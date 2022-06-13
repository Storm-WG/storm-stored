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

use microservices::rpc;
use storedrpc::{FailureCode, Reply};

#[derive(Clone, PartialEq, Eq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum LaunchError {
    #[from]
    #[display(inner)]
    Database(sled::Error),
}

impl microservices::error::Error for LaunchError {}

#[derive(Clone, PartialEq, Eq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum ServerError {
    #[from]
    #[display(inner)]
    Database(sled::Error),

    /// unknown database table '{0}'
    UnknownTable(String),

    #[from]
    #[display(inner)]
    Encoding(strict_encoding::Error),
}

impl microservices::error::Error for ServerError {}

impl From<ServerError> for Reply {
    fn from(err: ServerError) -> Self {
        let code = match err {
            ServerError::Database(_) => FailureCode::Database,
            ServerError::UnknownTable(_) => FailureCode::Database,
            ServerError::Encoding(_) => FailureCode::Encoding,
        };
        Reply::Failure(rpc::Failure {
            code: code.into(),
            info: err.to_string(),
        })
    }
}
