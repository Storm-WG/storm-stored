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
use microservices::rpc::ServerError;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[display(doc_comments)]
pub enum FailureCode {
    /// Catch-all
    #[display("unknown error")]
    Unknown = 0xFFF,

    /// internal database error
    Database = 0x01,

    /// internal encoding erorr
    Encoding = 0x02,
}

impl From<u16> for FailureCode {
    fn from(value: u16) -> Self {
        match value {
            _ => FailureCode::Unknown,
        }
    }
}

impl From<FailureCode> for u16 {
    fn from(code: FailureCode) -> Self { code as u16 }
}

impl From<FailureCode> for rpc::FailureCode<FailureCode> {
    fn from(code: FailureCode) -> Self { rpc::FailureCode::Other(code) }
}

impl From<FailureCode> for rpc::Failure<FailureCode> {
    fn from(code: FailureCode) -> Self {
        rpc::Failure {
            code: code.into(),
            info: code.to_string(),
        }
    }
}

impl From<FailureCode> for ServerError<FailureCode> {
    fn from(code: FailureCode) -> Self { ServerError::ServerFailure(code.into()) }
}

impl rpc::FailureCodeExt for FailureCode {}
