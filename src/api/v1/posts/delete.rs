// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;
use crate::api::v1::Limit;
use derive_builder::Builder;

/// Create a Delete endpoint for posts.
///
/// <https://pinboard.in/api/#posts_delete>
///
/// # Arguments
/// This builder takes four optional arguments.
/// * `url` - Return the bookmark for the URL
///
/// # Example
/// ```rust
/// # fn main() {
/// # use crate::pinboard_rs::api::v1::posts::Delete;
/// # use crate::pinboard_rs::api::Endpoint;
/// # use url::Url;
/// let del_endpoint = Delete::builder()
///                    .url(Url::parse("https://example.com").unwrap())
///                    .build().unwrap();
/// assert_eq!(del_endpoint.endpoint(), "v1/posts/delete");
/// # }
/// ```
#[derive(Debug, Clone, Builder)]
pub struct Delete {
    /// The bookmark to delete
    url: url::Url,
}

impl Delete {
    /// Create a builder for the endpoint
    pub fn builder() -> DeleteBuilder {
        DeleteBuilder::default()
    }
}

impl Endpoint for Delete {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/posts/delete".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("url", self.url.as_ref());
        params
    }
}

impl Limit for Delete {}

#[cfg(test)]
mod tests {
    use crate::api::v1::{posts::Delete, Limit};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    static URL: &str = "http://pinboard.test/";
    fn test_url() -> url::Url {
        url::Url::parse(URL).unwrap()
    }

    #[test]
    fn url_is_required() {
        let err = Delete::builder().build().unwrap_err();
        assert_eq!(&err.to_string(), "`url` must be initialized")
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/delete")
            .add_query_params(&[("url", URL)])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Delete::builder().url(test_url()).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
        assert_eq!(Delete::secs_between_calls(), 3)
    }
}
