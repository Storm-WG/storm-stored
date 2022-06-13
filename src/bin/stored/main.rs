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

//! Main executable for stored: storage microservice.

#[macro_use]
extern crate log;

mod opts;

use clap::Parser;
use microservices::error::BootstrapError;
use stored::{Config, LaunchError};

use crate::opts::Opts;

fn main() -> Result<(), BootstrapError<LaunchError>> {
    println!("stored: storage microservice");

    let opts = Opts::parse();
    trace!("Command-line arguments: {:?}", &opts);

    let mut config = Config {
        data_dir: opts.data_dir,
        rpc_endpoint: opts.rpc_endpoint,
        verbose: opts.verbose,
        databases: opts.db.iter().cloned().collect(),
    };
    trace!("Daemon configuration: {:?}", config);
    config.process();
    trace!("Processed configuration: {:?}", config);
    debug!("CTL RPC socket {}", config.rpc_endpoint);

    /*
    use self::internal::ResultExt;
    let (config_from_file, _) =
        internal::Config::custom_args_and_optional_files(std::iter::empty::<
            &str,
        >())
        .unwrap_or_exit();
     */

    debug!("Starting runtime ...");
    stored::service::run(config).expect("running stored runtime");

    unreachable!()
}
