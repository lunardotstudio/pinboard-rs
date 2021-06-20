// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Posts to the `awesome` endpoint.
#[derive(Debug, Clone, Copy, Builder)]
pub struct Auth {}

impl Auth {
    /// Create a builder for the endpoint.
    pub fn builder() -> AuthBuilder {
        AuthBuilder::default()
    }
}

impl Endpoint for Auth {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "auth".into()
    }
}
