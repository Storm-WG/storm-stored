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

use internet2::addr::ServiceAddr;
use store_rpc::STORED_RPC_ENDPOINT;
use storm::ChunkId;

/// Command-line tool for working with store daemon
#[derive(Parser, Clone, PartialEq, Eq, Debug)]
#[clap(name = "store-cli", bin_name = "store-cli", author, version)]
pub struct Opts {
    /// ZMQ socket for connecting daemon RPC interface.
    ///
    /// Socket can be either TCP address in form of `<ipv4 | ipv6>:<port>` – or a path
    /// to an IPC file.
    ///
    /// Defaults to `127.0.0.1:62962`.
    #[clap(
        short,
        long,
        global = true,
        default_value = STORED_RPC_ENDPOINT,
        env = "STORED_RPC_ENDPOINT"
    )]
    pub connect: ServiceAddr,

    /// Set verbosity level.
    ///
    /// Can be used multiple times to increase verbosity.
    #[clap(short, long, global = true, parse(from_occurrences))]
    pub verbose: u8,

    /// Command to execute
    #[clap(subcommand)]
    pub command: Command,
}

/// Command-line commands:
#[derive(Subcommand, Clone, PartialEq, Eq, Debug, Display)]
pub enum Command {
    /// Stores file into database
    #[display("store '{table}' '{file:?}'")]
    Store {
        /// Database table to store file in
        table: String,

        /// File to put into database. If no file is given, data are read from
        /// STDIN.
        file: Option<PathBuf>,
    },

    /// Retrieves file from the database and outputs it into the provided
    /// file name, or onto stdout if no output file is specified.
    ///
    /// The output file, if exists, gets truncated/overwritten.
    #[display("retrieve '{table}' {chunk_id}")]
    Retrieve {
        /// Database table to request file.
        table: String,

        /// Information (file chunk) identifier returned before with the `store`
        /// command.
        chunk_id: ChunkId,

        /// File for output. The data are printed to stdout if no file is given.
        output: Option<PathBuf>,
    },
}
