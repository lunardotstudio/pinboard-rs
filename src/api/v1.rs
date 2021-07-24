// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Pinboard API V1 endpoint structures
//!

pub mod notes;
pub mod posts;
pub mod tags;
pub mod user;

pub trait Limit {
    /// By default, the endoints are allowed calls every 3 seconds
    /// https://pinboard.in/api#limits
    fn secs_between_calls() -> usize {
        3
    }
}
