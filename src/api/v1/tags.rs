// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Builders for tags endpoints.
mod delete;
mod get;
mod rename;

#[doc(inline)]
pub use self::delete::Delete;
#[doc(inline)]
pub use self::get::Get;
#[doc(inline)]
pub use self::rename::Rename;
