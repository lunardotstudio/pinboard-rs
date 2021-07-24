// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::module_inception)]

//! General endpoints
//!
//! These endpoints are used for general site interactions

mod auth;
mod awesome;
mod hello;
mod last_update;

pub use self::auth::Auth;
pub use self::awesome::Awesome;
pub use self::hello::Hello;
pub use self::last_update::LastUpdate;
