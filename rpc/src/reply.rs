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

#![allow(clippy::clone_on_copy)] // Caused by Api derivation on Reply type

use std::collections::BTreeSet;

use amplify::Slice32;
use internet2::presentation;
use microservices::rpc;
use microservices::rpc::ServerError;
use storm::{Chunk, ChunkId};

use crate::FailureCode;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Display, From)]
#[derive(Api)]
#[api(encoding = "strict")]
#[non_exhaustive]
pub enum Reply {
    // Responses to CLI
    // ----------------
    #[api(type = 0x0001)]
    #[display("success({0})")]
    Success,

    #[api(type = 0x0000)]
    #[display("failure({0:#})")]
    #[from]
    Failure(rpc::Failure<FailureCode>),

    #[api(type = 0x00a1)]
    #[display("tables(...)")]
    Tables(BTreeSet<String>),

    #[api(type = 0x00a3)]
    #[display("count(...)")]
    Count(u64),

    #[api(type = 0x0011)]
    #[display("chunk_id({0})")]
    ChunkId(ChunkId),

    #[api(type = 0x0010)]
    #[display("chunk(...)")]
    Chunk(Chunk),

    #[api(type = 0x0013)]
    #[display("ids(...)")]
    Ids(BTreeSet<ChunkId>),

    #[api(type = 0x0012)]
    #[display("key_absent({0})")]
    KeyAbsent(Slice32),
}

impl rpc::Reply for Reply {}

impl From<presentation::Error> for Reply {
    fn from(err: presentation::Error) -> Self {
        Reply::Failure(rpc::Failure {
            code: rpc::FailureCode::Presentation,
            info: format!("{}", err),
        })
    }
}

impl Reply {
    pub fn success_or_failure(self) -> Result<(), ServerError<FailureCode>> {
        match self {
            Reply::Success => Ok(()),
            Reply::Failure(failure) => Err(failure.into()),
            _ => Err(ServerError::UnexpectedServerResponse),
        }
    }
}
