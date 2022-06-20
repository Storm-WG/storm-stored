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

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn open_file_or_stdin(filename: Option<impl AsRef<Path>>) -> Result<Box<dyn Read>, io::Error> {
    Ok(match filename {
        Some(filename) => {
            let file = fs::File::open(filename)?;
            Box::new(file)
        }
        None => Box::new(io::stdin()),
    })
}

pub fn open_file_or_stdout(
    filename: Option<impl AsRef<Path>>,
) -> Result<Box<dyn Write>, io::Error> {
    Ok(match filename {
        Some(filename) => {
            let file = fs::File::create(filename)?;
            Box::new(file)
        }
        None => Box::new(io::stdout()),
    })
}

pub fn read_file_or_stdin(filename: Option<impl AsRef<Path>>) -> Result<Vec<u8>, io::Error> {
    let mut reader = open_file_or_stdin(filename)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn write_file_or_stdout(
    data: impl AsRef<[u8]>,
    filename: Option<impl AsRef<Path>>,
) -> Result<(), io::Error> {
    let mut writer = open_file_or_stdout(filename)?;
    writer.write_all(data.as_ref())
}
