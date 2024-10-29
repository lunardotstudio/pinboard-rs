// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;
use crate::api::v1::Limit;
use derive_builder::Builder;

/// Create an Update endpoint for posts.
///
/// <https://pinboard.in/api/#posts_update>
///
/// This endpoint returns the most recent time a bookmark was added, updated, or deleted.
/// The guidance is to use this before All to see if a call to get all bookmarks is necessary.
///
/// # Example
/// ```rust
/// # fn main() {
/// # use crate::pinboard_rs::api::v1::posts::Update;
/// # use crate::pinboard_rs::api::Endpoint;
/// let upd_endpoint = Update::builder().build().unwrap();
/// assert_eq!(upd_endpoint.endpoint(), "v1/posts/update");
/// # }
/// ```
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

impl Limit for Update {}

#[cfg(test)]
mod tests {
    use crate::api::v1::{posts::Update, Limit};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/update")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Update::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
        assert_eq!(Update::secs_between_calls(), 3)
    }
}
