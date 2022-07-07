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

use std::collections::BTreeSet;

use internet2::addr::ServiceAddr;
use internet2::session::LocalSession;
use internet2::{
    CreateUnmarshaller, SendRecvMessage, TypedEnum, Unmarshall, Unmarshaller, ZmqSocketType,
};
use microservices::rpc::ServerError;
use microservices::ZMQ_CONTEXT;
use storm::{Chunk, ChunkId, TryFromChunk, TryToChunk};

use crate::{FailureCode, PrimaryKey, Reply, Request, RetrieveReq, StoreReq};

pub struct Client {
    // TODO: Replace with RpcSession once its implementation is completed
    session_rpc: LocalSession,
    unmarshaller: Unmarshaller<Reply>,
}

impl Client {
    pub fn with(connect: &ServiceAddr) -> Result<Self, ServerError<FailureCode>> {
        debug!("Initializing runtime");

        trace!("Connecting to store daemon at {}", connect);
        let session_rpc =
            LocalSession::connect(ZmqSocketType::Req, connect, None, None, &ZMQ_CONTEXT)?;
        Ok(Self {
            session_rpc,
            unmarshaller: Reply::create_unmarshaller(),
        })
    }

    pub fn use_table(&mut self, table: impl ToString) -> Result<(), ServerError<FailureCode>> {
        self.request(Request::Use(table.to_string()))?.success_or_failure()
    }

    pub fn list_tables(&mut self) -> Result<BTreeSet<String>, ServerError<FailureCode>> {
        match self.request(Request::Tables)? {
            Reply::Tables(tables) => Ok(tables),
            Reply::Failure(failure) => Err(failure.into()),
            _ => Err(ServerError::UnexpectedServerResponse),
        }
    }

    pub fn count(&mut self, table: impl ToString) -> Result<u64, ServerError<FailureCode>> {
        match self.request(Request::Count(table.to_string()))? {
            Reply::Count(count) => Ok(count),
            Reply::Failure(failure) => Err(failure.into()),
            _ => Err(ServerError::UnexpectedServerResponse),
        }
    }

    pub fn store(
        &mut self,
        table: impl ToString,
        key: impl Into<PrimaryKey>,
        data: &impl TryToChunk,
    ) -> Result<ChunkId, ServerError<FailureCode>> {
        let table = table.to_string();
        let key = key.into();
        trace!("Store object with id {}", key);
        let chunk = data.try_to_chunk().map_err(|_| FailureCode::Encoding)?;
        let reply = self.request(Request::Store(StoreReq { table, key, chunk }))?;
        match reply {
            Reply::ChunkId(chunk_id) => Ok(chunk_id),
            Reply::Failure(failure) => {
                warn!("Failure storing object with id {}", key);
                Err(failure.into())
            }
            _ => Err(ServerError::UnexpectedServerResponse),
        }
    }

    pub fn retrieve<D>(
        &mut self,
        table: impl ToString,
        key: impl Into<PrimaryKey>,
    ) -> Result<Option<D>, ServerError<FailureCode>>
    where
        D: TryFromChunk,
    {
        let table = table.to_string();
        let key = key.into();
        trace!("Retrieve object with id {}", key);
        let reply = self.request(Request::Retrieve(RetrieveReq { table, key }))?;
        match reply {
            Reply::Chunk(chunk) => D::try_from_chunk(chunk)
                .map_err(|_| FailureCode::Encoding)
                .map_err(ServerError::from)
                .map(Some),
            Reply::KeyAbsent(_) => {
                warn!("Object with id {} is not found", key);
                Ok(None)
            }
            _ => Err(ServerError::UnexpectedServerResponse),
        }
    }

    pub fn retrieve_chunk(
        &mut self,
        table: impl ToString,
        key: impl Into<PrimaryKey>,
    ) -> Result<Option<Chunk>, ServerError<FailureCode>> {
        self.retrieve(table, key)
    }

    pub fn ids(&mut self, table: String) -> Result<BTreeSet<ChunkId>, ServerError<FailureCode>> {
        let reply = self.request(Request::ListIds(table))?;
        match reply {
            Reply::Ids(ids) => Ok(ids),
            Reply::KeyAbsent(_) => Ok(empty!()),
            _ => Err(ServerError::UnexpectedServerResponse),
        }
    }

    fn request(&mut self, request: Request) -> Result<Reply, ServerError<FailureCode>> {
        trace!("Sending request to the server: {:?}", request);
        let data = request.serialize();
        trace!("Raw request data ({} bytes): {:02X?}", data.len(), data);
        self.session_rpc.send_raw_message(&data)?;
        trace!("Awaiting reply");
        let raw = self.session_rpc.recv_raw_message()?;
        trace!("Got reply ({} bytes), parsing: {:02X?}", raw.len(), raw);
        let reply = self.unmarshaller.unmarshall(raw.as_slice())?;
        trace!("Reply: {:?}", reply);
        Ok((&*reply).clone())
    }
}
