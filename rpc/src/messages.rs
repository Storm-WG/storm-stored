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

use std::collections::{BTreeMap, HashSet};
use std::fmt::{self, Debug, Display, Formatter};
use std::io;
use std::iter::FromIterator;
use std::str::FromStr;
use std::time::Duration;

use amplify::{Slice32, ToYamlString, Wrapper};
use internet2::addr::InetSocketAddr;
use internet2::{NodeAddr, RemoteNodeAddr, RemoteSocketAddr};
use microservices::rpc_connection;
#[cfg(feature = "serde")]
use serde_with::{DisplayFromStr, DurationSeconds, Same};
use strict_encoding::{StrictDecode, StrictEncode};
use wallet::address::AddressCompat;

/// RPC API requests between storage daemon and clients.
#[derive(Clone, Debug, Display, From, Api)]
#[api(encoding = "strict")]
#[non_exhaustive]
pub enum RpcMsg {
}

impl rpc_connection::Request for BusMsg {}
