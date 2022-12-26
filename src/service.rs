// Storage daemon (stored): microservice frontend for different storage backends
// used in LNP/BP nodes.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::collections::{BTreeSet, HashMap};

use amplify::Slice32;
use bitcoin_hashes::Hash;
use commit_verify::commit_encode::ConsensusCommit;
use internet2::session::LocalSession;
use internet2::{
    CreateUnmarshaller, SendRecvMessage, TypedEnum, Unmarshall, Unmarshaller, ZmqSocketType,
};
use microservices::error::BootstrapError;
use microservices::node::TryService;
use microservices::rpc::ClientError;
use microservices::ZMQ_CONTEXT;
use store_rpc::{CheckUnknownReq, InsertReq, PrimaryKey, Reply, Request, RetrieveReq, StoreReq};
use storm::{Chunk, ChunkId};
use strict_encoding::{StrictDecode, StrictEncode};

use crate::{Config, DaemonError, LaunchError, STORED_STORAGE_FILE};

pub fn run(config: Config) -> Result<(), BootstrapError<LaunchError>> {
    let runtime = Runtime::init(config)?;

    runtime.run_or_panic("stored");

    Ok(())
}

pub struct Runtime {
    /// Stored sessions
    pub(super) session_rpc: LocalSession,

    /// Unmarshaller instance used for parsing RPC request
    pub(super) unmarshaller: Unmarshaller<Request>,

    pub(super) db: sled::Db,

    pub(super) trees: HashMap<String, sled::Tree>,
}

impl Runtime {
    pub fn init(config: Config) -> Result<Self, BootstrapError<LaunchError>> {
        // debug!("Initializing storage provider {:?}", config.storage_conf());
        // let storage = storage::FileDriver::with(config.storage_conf())?;

        debug!("Opening RPC API socket {}", config.rpc_endpoint);
        let session_rpc = LocalSession::connect(
            ZmqSocketType::Rep,
            &config.rpc_endpoint,
            None,
            None,
            &ZMQ_CONTEXT,
        )?;

        let (db, trees) = Self::init_db(&config)?;

        info!("Stored runtime started successfully");

        Ok(Self {
            session_rpc,
            unmarshaller: Request::create_unmarshaller(),
            db,
            trees,
        })
    }

    fn init_db(config: &Config) -> Result<(sled::Db, HashMap<String, sled::Tree>), LaunchError> {
        let mut db_path = config.data_dir.clone();
        db_path.push(STORED_STORAGE_FILE);
        debug!("Opening database at {}", db_path.display());
        let db = sled::open(db_path)?;
        let trees = config
            .databases
            .iter()
            .map(|name| db.open_tree(name).map(|tree| (name.clone(), tree)))
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok((db, trees))
    }
}

impl TryService for Runtime {
    type ErrorType = ClientError;

    fn try_run_loop(mut self) -> Result<(), Self::ErrorType> {
        loop {
            match self.run() {
                Ok(_) => debug!("API request processing complete"),
                Err(err) => {
                    error!("Error processing API request: {}", err);
                    Err(err)?;
                }
            }
        }
    }
}

impl Runtime {
    fn run(&mut self) -> Result<(), ClientError> {
        trace!("Awaiting for ZMQ RPC requests...");
        let raw = self.session_rpc.recv_raw_message()?;
        let reply = self.rpc_process(raw).unwrap_or_else(|err| err);
        trace!("Preparing ZMQ RPC reply: {:?}", reply);
        let data = reply.serialize();
        trace!("Sending {} bytes back to the client over ZMQ RPC", data.len());
        self.session_rpc.send_raw_message(&data)?;
        Ok(())
    }
}

impl Runtime {
    pub(crate) fn rpc_process(&mut self, raw: Vec<u8>) -> Result<Reply, Reply> {
        trace!("Got {} bytes over ZMQ RPC", raw.len());
        let request = (*self.unmarshaller.unmarshall(raw.as_slice())?).clone();
        debug!("Received ZMQ RPC request #{}: {}", request.get_type(), request);
        match request {
            Request::Use(table) => self.use_table(table),
            Request::Tables => self.list_tables(),
            Request::Count(table) => self.count(table),
            Request::Store(StoreReq { table, key, chunk }) => self.store(table, key, chunk),
            Request::Retrieve(RetrieveReq { table, key }) => self.retrieve(table, key),
            Request::Insert(InsertReq { table, key, item }) => self.insert(table, key, item),
            Request::ListIds(table) => self.list_ids(table),
            Request::CheckUnknown(CheckUnknownReq { table, ids }) => self.filter_ids(table, ids),
        }
        .map_err(Reply::from)
    }

    fn use_table(&mut self, table: String) -> Result<Reply, DaemonError> {
        let tree = self.db.open_tree(&table)?;
        self.trees.insert(table, tree);
        Ok(Reply::Success)
    }

    fn list_tables(&self) -> Result<Reply, DaemonError> {
        let tables = self.trees.keys().cloned().collect();
        Ok(Reply::Tables(tables))
    }

    fn count(&self, table: String) -> Result<Reply, DaemonError> {
        let tree = self.trees.get(&table).ok_or(DaemonError::UnknownTable(table))?;
        let count = tree.len();
        Ok(Reply::Count(count as u64))
    }

    fn store(
        &self,
        table: String,
        key: impl PrimaryKey,
        chunk: Chunk,
    ) -> Result<Reply, DaemonError> {
        let tree = self.trees.get(&table).ok_or(DaemonError::UnknownTable(table))?;
        let chunk_id = chunk.consensus_commit();
        tree.insert(key.into_slice32(), chunk.as_ref())?;
        tree.flush()?;
        Ok(Reply::ChunkId(chunk_id))
    }

    fn retrieve(&self, table: String, key: impl PrimaryKey) -> Result<Reply, DaemonError> {
        let key = key.into_slice32();
        let tree = self.trees.get(&table).ok_or(DaemonError::UnknownTable(table))?;
        Ok(match tree.get(key)? {
            None => Reply::KeyAbsent(key),
            Some(data) => Reply::Chunk(data.as_ref().try_into()?),
        })
    }

    fn insert(
        &self,
        table: String,
        key: impl PrimaryKey,
        item: Slice32,
    ) -> Result<Reply, DaemonError> {
        let key = key.into_slice32();
        let tree = self.trees.get(&table).ok_or(DaemonError::UnknownTable(table))?;
        let data = tree.get(key)?.unwrap_or_default();
        let mut set = if data.is_empty() {
            BTreeSet::new()
        } else {
            BTreeSet::<Slice32>::strict_deserialize(data)?
        };
        set.insert(item);
        tree.insert(key, set.strict_serialize()?)?;
        tree.flush()?;
        Ok(Reply::Success)
    }

    fn list_ids(&self, table: String) -> Result<Reply, DaemonError> {
        let tree = self.trees.get(&table).ok_or(DaemonError::UnknownTable(table))?;
        let keys = tree
            .range::<&[u8], _>(..)
            .map(|res| match res {
                Ok((ivec, _)) => Ok(ChunkId::from_slice(&ivec)
                    .map_err(|_| sled::Error::ReportableBug(s!("non-standard id")))?),
                Err(e) => Err(e),
            })
            .collect::<Result<BTreeSet<_>, sled::Error>>()?;
        Ok(Reply::Ids(keys))
    }

    fn filter_ids(&self, table: String, mut ids: BTreeSet<ChunkId>) -> Result<Reply, DaemonError> {
        let tree = self.trees.get(&table).ok_or(DaemonError::UnknownTable(table))?;
        // TODO: Improve efficiency by restricting the range
        for res in tree.range::<&[u8], _>(..) {
            let (ivec, _) = res?;
            let id = ChunkId::from_slice(&ivec).map_err(|_| {
                DaemonError::Encoding(strict_encoding::Error::DataIntegrityError(s!(
                    "invalid chunk id data"
                )))
            })?;
            ids.remove(&id);
        }
        Ok(Reply::Ids(ids))
    }
}
