// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]

//! API endpoint structures
//!
//! The types provided in this module are intended to easily create
//! endpoint calls. Each endpoint employs a builder pattern to allow
//! easy construction of a call.

mod client;
mod endpoint;
mod error;
mod params;
pub(crate) mod query;
mod ignore;

pub mod v1;
pub mod v2;

pub mod endpoint_prelude;

pub use self::client::AsyncClient;
pub use self::client::Client;
pub use self::client::RestClient;

pub use self::endpoint::Endpoint;

pub use self::query::AsyncQuery;
pub use self::query::Query;

pub use self::params::QueryParams;
pub use self::params::ParamValue;

pub use self::error::ApiError;
pub use self::error::BodyError;

pub use self::ignore::ignore;
pub use self::ignore::Ignore;
