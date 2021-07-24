// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;
use crate::api::v1::Limit;
use chrono::NaiveDate;

/// Query the `update` endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Get<'a> {
    /// Tag filter
    #[builder(setter(into), default)]
    tag: Option<Cow<'a, str>>,
    /// Return results on the provided day
    #[builder(default)]
    dt: Option<NaiveDate>,
    /// Return bookmark for this url,
    #[builder(default)]
    url: Option<url::Url>,
    /// Include a change detection signature in results
    #[builder(default)]
    meta: Option<bool>,
}

impl<'a> Get<'a> {
    /// Create a builder for the endpoint
    pub fn builder() -> GetBuilder<'a> {
        GetBuilder::default()
    }
}

impl<'a> Endpoint for Get<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/posts/get".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("meta", self.meta.map(|x| if x { "yes" } else { "no" }))
            .push_opt("tag", self.tag.as_ref())
            .push_opt("url", self.url.as_ref())
            .push_opt("dt", self.dt);

        params
    }
}

impl<'a> Limit for Get<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::v1::{posts::Get, Limit};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};
    use chrono::NaiveDate;

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/get")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Get::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_meta() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/get")
            .add_query_params(&[("meta", "yes")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Get::builder().meta(true).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_url() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/get")
            .add_query_params(&[("url", "http://pinboard.in/")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let url = url::Url::parse("http://pinboard.in").unwrap();
        let endpoint = Get::builder().url(url).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/get")
            .add_query_params(&[("tag", "Tag1")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Get::builder().tag("Tag1").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_dt() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/get")
            .add_query_params(&[("dt", "2021-03-04")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Get::builder()
            .dt(NaiveDate::from_ymd(2021, 3, 4))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
        assert_eq!(Get::secs_between_calls(), 3)
    }
}
