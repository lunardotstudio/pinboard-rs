// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Query the `update` endpoint.
#[derive(Debug, Clone, Copy, Builder)]
pub struct Update {}

impl Update {
    /// Create a builder for the endpoint
    pub fn builder() -> UpdateBuilder {
        UpdateBuilder::default()
    }
}

impl Endpoint for Update {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/posts/update".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::v1::posts::Update;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("v1/posts/update").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint,"");

        let endpoint = Update::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
