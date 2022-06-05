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

use internet2::presentation;
use microservices::rpc::Failure;
use microservices::rpc_connection;

use crate::Error;

#[derive(Clone, Debug, Display, From, Api)]
#[api(encoding = "strict")]
#[non_exhaustive]
pub enum Reply {
    // Responses to CLI
    // ----------------
    #[api(type = 0x0002)]
    #[display("success({0})")]
    Success,

    #[api(type = 0x0000)]
    #[display("failure({0:#})")]
    #[from]
    Failure(Failure),
}

impl rpc_connection::Reply for Reply {}

impl From<presentation::Error> for Reply {
    fn from(err: presentation::Error) -> Self {
        // TODO: Save error code taken from `Error::to_value()` after
        //       implementation of `ToValue` trait and derive macro for enums
        Reply::Failure(Failure {
            code: 0,
            info: format!("{}", err),
        })
    }
}

impl From<Error> for Failure {
    fn from(err: Error) -> Self {
        match err {
            // Error::ServerFailure(failure) => failure,
            err => Failure {
                code: 1,
                info: err.to_string(),
            },
        }
    }
}

impl From<Error> for Reply {
    fn from(err: Error) -> Self { Reply::Failure(err.into()) }
}
