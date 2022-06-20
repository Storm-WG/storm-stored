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

use amplify::Slice32;
pub use client::Client;
pub use error::FailureCode;
pub use reply::Reply;
pub use request::{Request, RetrieveReq, StoreReq};

#[cfg(any(target_os = "linux"))]
pub const STORED_DATA_DIR: &str = "~/.storm";
#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
pub const STORED_DATA_DIR: &str = "~/.storm";
#[cfg(target_os = "macos")]
pub const STORED_DATA_DIR: &str = "~/Library/Application Support/Storm Node";
#[cfg(target_os = "windows")]
pub const STORED_DATA_DIR: &str = "~\\AppData\\Local\\Storm Node";
#[cfg(target_os = "ios")]
pub const STORED_DATA_DIR: &str = "~/Documents";
#[cfg(target_os = "android")]
pub const STORED_DATA_DIR: &str = ".";

pub const STORED_RPC_ENDPOINT: &str = const_format::concatcp!(STORED_DATA_DIR, "/store");

pub type PrimaryKey = Slice32;
