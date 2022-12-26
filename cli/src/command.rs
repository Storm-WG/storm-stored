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

use microservices::cli;
use microservices::rpc::ServerError;
use microservices::shell::Exec;
use store_rpc::{Client, FailureCode};
use storm::Chunk;

use crate::{Command, Opts};

impl Exec for Opts {
    type Client = Client;
    type Error = ServerError<FailureCode>;

    fn exec(self, client: &mut Self::Client) -> Result<(), Self::Error> {
        debug!("Performing {:?} ... ", self.command);
        match self.command {
            Command::Use { table } => {
                eprintln!("Using table {}", table);
                client.use_table(table)?;
            }
            Command::Tables => {
                eprintln!("Listing tables:");
                let tables = client.list_tables()?;
                for table in tables {
                    println!("{}", table);
                }
            }
            Command::Count { table } => {
                eprint!("Database table `{}` contains ", table);
                eprintln!("{} object(s)", client.count(table)?);
            }
            Command::Store {
                table: db,
                key,
                file,
            } => {
                let data = cli::read_file_or_stdin(file).expect("unable to read the file");
                let chunk = Chunk::try_from(&data)?;
                let chunk_id = client.store(db, key, &chunk)?;
                eprint!("Stored chunk id ");
                println!("{}", chunk_id);
            }
            Command::Retrieve { table, key, output } => match client.retrieve_chunk(table, key)? {
                Some(chunk) => {
                    eprintln!("success");
                    let output_filename = output
                        .as_deref()
                        .map(|f| f.display().to_string())
                        .unwrap_or_else(|| s!("STDOUT"));
                    eprint!("Writing to {} ... ", output_filename);
                    cli::write_file_or_stdout(chunk, output).expect("unable to write to the file");
                    eprintln!("success");
                }
                None => {
                    eprintln!("unknown chunk");
                }
            },
            Command::Ids { table } => {
                eprintln!("success");
                eprintln!("Found ids:");
                let ids = client.ids(table)?;
                for id in ids {
                    println!("{}", id);
                }
            }
        }
        eprintln!();
        Ok(())
    }
}
