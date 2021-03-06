// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Native contracts useful for Parity. These are type-safe wrappers
//! autogenerated at compile-time from Ethereum ABIs, and can be instantiated
//! given any closure which can dispatch calls to them asynchronously.

extern crate futures;
extern crate byteorder;
extern crate ethabi;
extern crate ethcore_bigint as bigint;

mod key_server_set;
mod registry;
mod urlhint;
mod service_transaction;
mod secretstore_acl_storage;
mod validator_set;
mod validator_report;

pub mod test_contracts;

pub use self::key_server_set::KeyServerSet;
pub use self::registry::Registry;
pub use self::urlhint::Urlhint;
pub use self::service_transaction::ServiceTransactionChecker;
pub use self::secretstore_acl_storage::SecretStoreAclStorage;
pub use self::validator_set::ValidatorSet;
pub use self::validator_report::ValidatorReport;
