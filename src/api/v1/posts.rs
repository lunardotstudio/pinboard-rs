// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(clippy::module_inception)]

//! Builders to interact with the set of post endpoints.
//!
mod add;
mod all;
mod dates;
mod delete;
mod get;
mod recent;
mod suggest;
mod update;

pub use self::add::Add;
pub use self::all::All;
pub use self::dates::Dates;
pub use self::delete::Delete;
pub use self::get::Get;
pub use self::recent::Recent;
pub use self::suggest::Suggest;
pub use self::update::Update;
