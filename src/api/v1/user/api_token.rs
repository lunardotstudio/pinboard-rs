// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;
use crate::api::v1::Limit;
use derive_builder::Builder;

/// Query the `api_token` endpoint.
#[derive(Debug, Clone, Copy, Builder)]
pub struct ApiToken {}

impl ApiToken {
    /// reate a builder for the endpoint
    pub fn builder() -> ApiTokenBuilder {
        ApiTokenBuilder::default()
    }
}

impl Endpoint for ApiToken {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/user/api_token".into()
    }
}

impl Limit for ApiToken {}

#[cfg(test)]
mod tests {
    use crate::api::v1::{user::ApiToken, Limit};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/user/api_token")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ApiToken::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
        assert_eq!(ApiToken::secs_between_calls(), 3)
    }
}
