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

use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn open_file_or_stdin(
    filename: Option<impl AsRef<Path>>,
) -> Result<Box<dyn io::Read>, io::Error> {
    Ok(match filename {
        Some(filename) => {
            let file = fs::File::open(filename)?;
            Box::new(file)
        }
        None => Box::new(io::stdin()),
    })
}

pub fn read_file_or_stdin(filename: Option<impl AsRef<Path>>) -> Result<Vec<u8>, io::Error> {
    let mut reader = open_file_or_stdin(filename)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
