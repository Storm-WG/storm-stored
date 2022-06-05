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

use microservices::shell::Exec;
use storedrpc::client::Client;
use storedrpc::{self, Error};

use crate::{Command, Opts};

impl Exec for Opts {
    type Client = Client;
    type Error = Error;

    fn exec(self, _runtime: &mut Self::Client) -> Result<(), Self::Error> {
        debug!("Performing {:?}", self.command);
        match self.command {
            Command::None => {}
        }
        Ok(())
    }
}
