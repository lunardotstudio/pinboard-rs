// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// #![warn(missing_docs)]

#[allow(clippy::needless_doctest_main)]
#[doc = include_str!("../README.md")]
#[cfg(feature = "async")]
mod async_pinboard;
mod auth;
mod pinboard;

pub mod api;
pub mod types;

#[cfg(feature = "async")]
pub use crate::async_pinboard::AsyncPinboard;
pub use crate::pinboard::{Pinboard, PinboardError};

#[cfg(test)]
mod test;
