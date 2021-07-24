// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Builders to interact with the set of tags endpoints.
mod delete;
mod get;
mod rename;

pub use self::delete::Delete;
pub use self::get::Get;
pub use self::rename::Rename;
