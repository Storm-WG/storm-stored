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

use std::collections::HashMap;

use commit_verify::commit_encode::ConsensusCommit;
use internet2::session::LocalSession;
use internet2::{
    CreateUnmarshaller, SendRecvMessage, TypedEnum, Unmarshall, Unmarshaller, ZmqSocketType,
};
use microservices::error::BootstrapError;
use microservices::node::TryService;
use microservices::rpc::ClientError;
use storedrpc::{ChunkInfo, Reply, Request, StoreReq};
use storm::{Chunk, ChunkId};

use crate::{Config, DaemonError, LaunchError};

pub fn run(config: Config) -> Result<(), BootstrapError<LaunchError>> {
    let runtime = Runtime::init(config)?;

    runtime.run_or_panic("stored");

    Ok(())
}

pub struct Runtime {
    /// Original configuration object
    pub(super) config: Config,

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
        let ctx = zmq::Context::new();
        let session_rpc =
            LocalSession::connect(ZmqSocketType::Rep, &config.rpc_endpoint, None, None, &ctx)?;

        let (db, trees) = Self::init_db(&config)?;

        info!("Stored runtime started successfully");

        Ok(Self {
            config,
            session_rpc,
            unmarshaller: Request::create_unmarshaller(),
            db,
            trees,
        })
    }

    fn init_db(config: &Config) -> Result<(sled::Db, HashMap<String, sled::Tree>), LaunchError> {
        let mut db_path = config.data_dir.clone();
        db_path.push("sled.db");
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
        let request = (&*self.unmarshaller.unmarshall(raw.as_slice())?).clone();
        debug!("Received ZMQ RPC request #{}: {}", request.get_type(), request);
        match request {
            Request::Store(StoreReq { db, chunk }) => self.store(db, chunk),
            Request::Retrieve(ChunkInfo { db, chunk_id }) => self.retrieve(db, chunk_id),
        }
        .map_err(Reply::from)
    }

    fn store(&self, db: String, chunk: Chunk) -> Result<Reply, DaemonError> {
        let tree = self.trees.get(&db).ok_or(DaemonError::UnknownTable(db))?;
        let chunk_id = chunk.consensus_commit();
        tree.insert(chunk_id, chunk.as_ref())?;
        Ok(Reply::ChunkId(chunk_id))
    }

    fn retrieve(&self, db: String, chunk_id: ChunkId) -> Result<Reply, DaemonError> {
        let tree = self.trees.get(&db).ok_or(DaemonError::UnknownTable(db))?;
        Ok(match tree.get(chunk_id)? {
            None => Reply::ChunkAbsent(chunk_id),
            Some(data) => Reply::Chunk(data.as_ref().try_into()?),
        })
    }
}
