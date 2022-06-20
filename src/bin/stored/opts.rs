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

use std::path::PathBuf;

use clap::{Parser, ValueHint};
use internet2::addr::ServiceAddr;
use store_rpc::STORED_DATA_DIR;

pub const STORED_CONFIG: &str = "{data_dir}/stored.toml";
// We redefine constant here and do not use one from `store_rpc` since we need
// to update the default path if the daemon was provided with a custom
// `data_dir`.
const STORED_RPC_ENDPOINT: &str = "{data_dir}/store";

/// Command-line arguments
#[derive(Parser)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[clap(author, version, name = "stored", about = "stored: storage microservice daemon")]
pub struct Opts {
    /// Set verbosity level.
    ///
    /// Can be used multiple times to increase verbosity
    #[clap(short, long, global = true, parse(from_occurrences))]
    pub verbose: u8,

    /// Data directory path.
    ///
    /// Path to the directory that contains stored data, and where ZMQ RPC
    /// socket files are located
    #[clap(
        short,
        long,
        global = true,
        default_value = STORED_DATA_DIR,
        env = "STORED_DATA_DIR",
        value_hint = ValueHint::DirPath
    )]
    pub data_dir: PathBuf,

    /// ZMQ socket name/address for store daemon RPC interface.
    ///
    /// Internal interface for control PRC protocol communications.
    #[clap(
        short = 'x',
        long,
        env = "STORED_RPC_ENDPOINT",
        value_hint = ValueHint::FilePath,
        default_value = STORED_RPC_ENDPOINT
    )]
    pub rpc_endpoint: ServiceAddr,

    /// Database table names to use.
    #[clap()]
    pub tables: Vec<String>,
}
