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

#![recursion_limit = "256"]

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding;
#[macro_use]
extern crate internet2;
#[macro_use]
extern crate log;

#[cfg(feature = "serde")]
extern crate serde_crate as serde;
//#[cfg(feature = "serde")]
//#[macro_use]
//extern crate serde_with;

pub mod client;
mod error;
mod reply;
mod request;

use std::borrow::Borrow;

use amplify::Slice32;
pub use client::Client;
pub use error::FailureCode;
pub use reply::Reply;
pub use request::{InsertReq, Request, RetrieveReq, StoreReq};

pub const STORED_RPC_ENDPOINT: &str = "0.0.0.0:60960";

pub trait PrimaryKey: Copy {
    fn into_array(self) -> [u8; 32];
    fn into_slice32(self) -> Slice32 { Slice32::from(self.into_array()) }
}

impl<W> PrimaryKey for W
where W: Copy + Borrow<[u8]>
{
    fn into_array(self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(self.borrow());
        buf
    }
}

#[cfg(test)]
mod test {
    use amplify::Slice32;
    use bitcoin_hashes::sha256::HashEngine;
    use bitcoin_hashes::{sha256, sha256t, Hash};

    use crate::PrimaryKey;

    #[test]
    fn primary_key_conversion() {
        #[derive(Copy, Clone)]
        pub struct Tag;
        impl sha256t::Tag for Tag {
            fn engine() -> HashEngine { sha256::Hash::engine() }
        }

        #[derive(Wrapper, Copy, Clone, Default, From)]
        #[wrapper(BorrowSlice)]
        pub struct Id(sha256t::Hash<Tag>);

        fn take_primary_key(_: impl PrimaryKey) {}

        take_primary_key(Slice32::default());
        take_primary_key(sha256::Hash::default());
        take_primary_key(Id::default());
    }
}
