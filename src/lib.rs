// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// TODO: Document API entities.
// #![warn(missing_docs)]

//! A library for communicating with Gitlab instances.

mod pinboard;
mod auth;

pub mod types;
pub mod api;

#[cfg(test)]
mod test;

pub use crate::pinboard::{AsyncPinboard, Pinboard, PinboardBuilder, PinboardError};
