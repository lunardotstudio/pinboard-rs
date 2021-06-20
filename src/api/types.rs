// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Types used across endpoints
use crate::api::ParamValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YesNo {
    /// Yes
    Yes,
    /// No
    No
}

impl Default for YesNo {
    fn default() -> Self {
        YesNo::No
    }
}
