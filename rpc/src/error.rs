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

use microservices::{esb, rpc};

use crate::ServiceId;

#[derive(Debug, Display, From, Error)]
#[display(doc_comments)]
#[non_exhaustive]
pub enum Error {
    /// ESB error: {0}
    #[from]
    Esb(esb::Error<ServiceId>),

    /// RPC error: {0}
    #[from]
    Rpc(rpc::Error),

    /// other error type with string explanation
    #[display(inner)]
    #[from(internet2::addr::NoOnionSupportError)]
    Other(String),
}

impl From<Error> for esb::Error<ServiceId> {
    fn from(err: Error) -> Self {
        match err {
            Error::Esb(err) => err,
            err => esb::Error::ServiceError(err.to_string()),
        }
    }
}

impl From<Error> for rpc::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Rpc(err) => err,
            err => rpc::Error::ServerFailure(rpc::Failure { code: 2000, info: err.to_string() }),
        }
    }
}
