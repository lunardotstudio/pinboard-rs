// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;
use crate::api::v1::Limit;
use derive_builder::Builder;

/// Create a Suggest endpoint for posts.
///
/// <https://pinboard.in/api/#posts_suggest>
///
/// # Arguments
/// This builder takes four optional arguments.
/// * `url` - Return the bookmark for the URL
///
/// # Example
/// ```rust
/// # fn main() {
/// # use crate::pinboard_rs::api::v1::posts::Suggest;
/// # use crate::pinboard_rs::api::Endpoint;
/// # use url::Url;
/// let suggest_endpoint = Suggest::builder()
///                        .url(Url::parse("http://example.com/").unwrap())
///                        .build().unwrap();
/// assert_eq!(suggest_endpoint.endpoint(), "v1/posts/suggest");
/// # }
/// ```
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Suggest {
    /// The bookmark to save
    url: url::Url,
}

impl Suggest {
    /// Create a builder for the endpoint
    pub fn builder() -> SuggestBuilder {
        SuggestBuilder::default()
    }
}

impl Endpoint for Suggest {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/posts/suggest".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("url", self.url.as_ref());

        params
    }
}

impl Limit for Suggest {}

#[cfg(test)]
mod tests {
    use crate::api::v1::{posts::Suggest, Limit};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    static URL: &str = "http://pinboard.test/";
    fn test_url() -> url::Url {
        url::Url::parse(URL).unwrap()
    }

    #[test]
    fn url_is_required() {
        let err = Suggest::builder().build().unwrap_err();
        assert_eq!(&err.to_string(), "`url` must be initialized")
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/suggest")
            .add_query_params(&[("url", URL)])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Suggest::builder().url(test_url()).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
        assert_eq!(Suggest::secs_between_calls(), 3)
    }
}
