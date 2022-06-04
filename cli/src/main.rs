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
// Coding conventions
#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    dead_code
    // missing_docs,
)]

//! Command-line interface to store daemon

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod command;
mod opts;

use clap::Parser;
use lnp_rpc::Client;
use microservices::shell::{Exec, LogLevel};

pub use crate::opts::{Command, Opts};

fn main() {
    println!("store-cli: command-line tool for working with LNP node");

    let opts = Opts::parse();
    LogLevel::from_verbosity_flag_count(opts.verbose).apply();

    trace!("Command-line arguments: {:?}", opts);

    let mut client = Client::with(&opts.connect).expect("Error initializing client");

    trace!("Executing command: {:?}", opts.command);
    opts.command.exec(&mut client).unwrap_or_else(|err| eprintln!("{}", err));
}
