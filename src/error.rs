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
use store_rpc::{FailureCode, Reply};

#[derive(Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum LaunchError {
    #[cfg(feature = "sled")]
    #[from]
    #[display(inner)]
    Database(sled::Error),

    #[cfg(feature = "redb")]
    #[from]
    #[display(inner)]
    Database(redb::Error),
}

impl microservices::error::Error for LaunchError {}

#[derive(Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum DaemonError {
    #[cfg(feature = "sled")]
    #[from]
    #[display(inner)]
    Database(sled::Error),

    #[cfg(feature = "redb")]
    #[from]
    #[display(inner)]
    Database(redb::Error),

    /// unknown database table '{0}'
    UnknownTable(String),

    #[from]
    #[display(inner)]
    Encoding(strict_encoding::Error),

    #[from]
    #[display(inner)]
    Hash(bitcoin_hashes::Error),
}

impl microservices::error::Error for DaemonError {}

impl From<DaemonError> for Reply {
    fn from(err: DaemonError) -> Self {
        let code = match err {
            DaemonError::Database(_) => FailureCode::Database,
            DaemonError::UnknownTable(_) => FailureCode::Database,
            DaemonError::Encoding(_) => FailureCode::Encoding,
            DaemonError::Hash(_) => FailureCode::Hash,
        };
        Reply::Failure(rpc::Failure {
            code: code.into(),
            info: err.to_string(),
        })
    }
}
