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

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate log;

mod config;
mod error;
pub mod service;
#[cfg(feature = "server")]
pub mod opts;

pub use config::Config;
pub use error::{DaemonError, LaunchError};

pub(crate) const STORED_STORAGE_FILE: &str = "sled.db";
