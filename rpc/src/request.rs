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

use amplify::Slice32;
use storm::{Chunk, ChunkId};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(Api)]
#[api(encoding = "strict")]
pub enum Request {
    /// Connects table in storage. If table is absent, creates one.
    #[api(type = 0xa0)]
    #[display("use({0})")]
    Use(String),

    #[api(type = 0xa1)]
    #[display("tables({0})")]
    Tables,

    #[api(type = 0xa3)]
    #[display("count({0})")]
    Count(String),

    #[api(type = 0x10)]
    #[display("store({0})")]
    Store(StoreReq),

    #[api(type = 0x12)]
    #[display("retrieve({0})")]
    Retrieve(RetrieveReq),

    #[api(type = 0x14)]
    #[display("insert({0})")]
    Insert(InsertReq),

    #[api(type = 0x16)]
    #[display("list_ids({0})")]
    ListIds(String),

    #[api(type = 0x18)]
    #[display("check_unknown({0})")]
    CheckUnknown(CheckUnknownReq),
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash, Display)]
#[derive(NetworkEncode, NetworkDecode)]
#[display("{table}, {key}, ...")]
pub struct StoreReq {
    pub table: String,
    pub key: Slice32,
    pub chunk: Chunk,
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash, Display)]
#[derive(NetworkEncode, NetworkDecode)]
#[display("{table}, {key}")]
pub struct RetrieveReq {
    pub table: String,
    pub key: Slice32,
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash, Display)]
#[derive(NetworkEncode, NetworkDecode)]
#[display("{table}, {key}, ...")]
pub struct InsertReq {
    pub table: String,
    pub key: Slice32,
    pub item: Slice32,
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash, Display)]
#[derive(NetworkEncode, NetworkDecode)]
#[display("{table}, ...")]
pub struct CheckUnknownReq {
    pub table: String,
    pub ids: BTreeSet<ChunkId>,
}
