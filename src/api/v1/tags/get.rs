// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;
use crate::api::v1::Limit;
use derive_builder::Builder;

/// Create a Get endpoint for tags.
///
/// <https://pinboard.in/api/#tags_get>
///
/// There are no arguments for this endpoint.
///
/// # Example
/// ```rust
/// # fn main() {
/// # use crate::pinboard_rs::api::v1::tags::Get;
/// # use crate::pinboard_rs::api::Endpoint;
/// let tags_endpoint = Get::builder().build().unwrap();
/// assert_eq!(tags_endpoint.endpoint(), "v1/tags/get");
/// # }
/// ```
#[derive(Debug, Clone, Copy, Builder)]
pub struct Get {}

impl Get {
    /// Create a builder for the endpoint
    pub fn builder() -> GetBuilder {
        GetBuilder::default()
    }
}

impl Endpoint for Get {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/tags/get".into()
    }
}

impl Limit for Get {}

#[cfg(test)]
mod tests {
    use crate::api::v1::{tags::Get, Limit};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/tags/get")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Get::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
        assert_eq!(Get::secs_between_calls(), 3)
    }
}
