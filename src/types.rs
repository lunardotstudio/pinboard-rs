// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! API entities
//!
//! These entities are exposed by Pinboard via its API.
//!
//! There are some places where Gitlab does not completely specify its types. This causes
//! problems when the types and names change inside of those. If found, issues should be filed
//! upstream.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Update {
    /// When the last update occured
    pub update_time: DateTime<Utc>,
}
