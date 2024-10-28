// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// TODO: Document API entities.
// #![warn(missing_docs)]

//! A library for communicating with Pinboard.in

#[cfg(feature = "asychronous")]
mod async_pinboard;
mod auth;
mod pinboard;

pub mod api;
pub mod types;

#[cfg(test)]
mod test;

pub use crate::pinboard::{Pinboard, PinboardError};

#[cfg(feature = "asychronous")]
pub use crate::async_pinboard::AsyncPinboard;
