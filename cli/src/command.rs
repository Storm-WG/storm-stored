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

use microservices::rpc::ServerError;
use microservices::shell::Exec;
use store_rpc::{ChunkInfo, Client, FailureCode, Reply, Request, StoreReq};
use storm::Chunk;

use crate::util::{read_file_or_stdin, write_file_or_stdout};
use crate::{Command, Opts};

impl Exec for Opts {
    type Client = Client;
    type Error = ServerError<FailureCode>;

    fn exec(self, runtime: &mut Self::Client) -> Result<(), Self::Error> {
        eprint!("Performing {:?} ... ", self.command);
        let reply = match self.command {
            Command::Store { db, file } => {
                let data = read_file_or_stdin(file).expect("unable to read the file");
                let chunk = Chunk::try_from(data.as_slice()).expect("file is too large");
                let reply = runtime.request(Request::Store(StoreReq { db, chunk }))?;
                Some(reply)
            }
            Command::Retrieve {
                db,
                chunk_id,
                output,
            } => {
                let reply = runtime.request(Request::Retrieve(ChunkInfo { db, chunk_id }))?;
                match reply {
                    Reply::Chunk(chunk) => {
                        eprintln!("success");
                        let output_filename = output
                            .as_deref()
                            .map(|f| f.display().to_string())
                            .unwrap_or(s!("STDOUT"));
                        eprint!("Writing to {} ... ", output_filename);
                        write_file_or_stdout(chunk, output).expect("unable to write to the file");
                        eprintln!("success");
                    }
                    Reply::ChunkAbsent(id) => {
                        eprintln!("unknown chunk");
                    }
                    _ => unreachable!("unexpected server response"),
                }
                None
            }
        };
        match reply {
            Some(Reply::Success) => eprintln!("success"),
            Some(Reply::Failure(failure)) => eprintln!("failure: {}", failure),
            None => {}
            _ => unreachable!("unknown server response"),
        }
        Ok(())
    }
}
