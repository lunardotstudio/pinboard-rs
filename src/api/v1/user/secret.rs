// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;
use crate::api::v1::Limit;
use derive_builder::Builder;

/// Create a Secret endpoint for users.
///
/// <https://pinboard.in/api/#user_secret>
///
/// This endpoint returns the user's secret RSS key.
///
/// # Arguments
/// There are no arguments for this endpoint.
///
/// # Example
/// ```rust
/// # fn main() {
/// # use crate::pinboard_rs::api::v1::user::Secret;
/// # use crate::pinboard_rs::api::Endpoint;
/// let secret_endpoint = Secret::builder().build().unwrap();
/// assert_eq!(secret_endpoint.endpoint(), "v1/user/secret");
/// # }
/// ```
#[derive(Debug, Clone, Copy, Builder)]
pub struct Secret {}

impl Secret {
    /// Create a builder for the endpoint
    pub fn builder() -> SecretBuilder {
        SecretBuilder::default()
    }
}

impl Endpoint for Secret {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/user/secret".into()
    }
}

impl Limit for Secret {}

#[cfg(test)]
mod tests {
    use crate::api::v1::{user::Secret, Limit};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/user/secret")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Secret::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
        assert_eq!(Secret::secs_between_calls(), 3)
    }
}
