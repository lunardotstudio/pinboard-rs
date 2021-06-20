// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(clippy::module_inception)]

//! Builders to interact with the set of post endpoints.
//!
mod update;
mod add;
mod delete;
mod get;
mod recent;
mod dates;
mod all;
mod suggest;

pub use self::update::Update;
pub use self::add::Add;
pub use self::delete::Delete;
pub use self::get::Get;
pub use self::recent::Recent;
pub use self::dates::Dates;
pub use self::all::All;
pub use self::suggest::Suggest;
