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

use internet2::presentation;
use microservices::rpc;
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

    #[api(type = 0x0011)]
    #[display("chunk_id({0})")]
    ChunkId(ChunkId),

    #[api(type = 0x0010)]
    #[display("chunk(...)")]
    Chunk(Chunk),

    #[api(type = 0x0013)]
    #[display("chunk_absent({0})")]
    ChunkAbsent(ChunkId),
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
