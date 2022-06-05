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

#[derive(Debug, Display, Error, From)]
#[display(inner)]
pub enum Error {
    #[from]
    Transport(internet2::transport::Error),

    #[from]
    Presentation(internet2::presentation::Error),

    #[from]
    Rpc(microservices::rpc::Failure),

    /// unexpected RPC API message; please check that the client version
    /// matches server
    UnexpectedApi,
}

impl microservices::error::Error for Error {}
