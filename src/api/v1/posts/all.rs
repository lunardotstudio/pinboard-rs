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
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct All<'a> {
    /// Filter by up to 3 tags
    #[builder(setter(into), default)]
    tags: Option<Cow<'a, [&'a str]>>,
    /// Offset value (default is 0)
    #[builder(default)]
    start: Option<u64>,
    /// Number of results to return (default all)
    #[builder(default)]
    results: Option<u64>,
    /// Return only bookmarks created after this time
    #[builder(default)]
    fromdt: Option<NaiveDate>,
    /// Return only bookmarks created before this time
    #[builder(default)]
    todt: Option<NaiveDate>,
    /// Include a change detection signature for each bookmark
    #[builder(default)]
    meta: Option<bool>,
}

impl<'a> AllBuilder<'a> {
    // Check that the tags to not exceed 100
    fn validate(&self) -> Result<(), String> {
        if let Some(ref cow) = self.tags {
            if let Some(xs) = cow {
                if xs.len() > 3 {
                    return Err(format!(
                        "Endpoint only accepts up to 3 tags (received {})",
                        xs.len()
                    ));
                }
            }
        }
        Ok(())
    }
}

impl<'a> All<'a> {
    /// Create a builder for the endpoint
    pub fn builder() -> AllBuilder<'a> {
        AllBuilder::default()
    }
}

impl<'a> Endpoint for All<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/posts/all".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("tag", self.tags.as_ref().map(|xs| xs.join(" ")))
            .push_opt("start", self.start)
            .push_opt("results", self.results)
            .push_opt("fromdt", self.fromdt)
            .push_opt("todt", self.todt)
            .push_opt("meta", self.meta.map(|x| if x { "yes" } else { "no" }));

        params
    }
}

impl<'a> Limit for All<'a> {
    fn secs_between_calls() -> usize {
        300
    }
}

#[cfg(test)]
mod tests {
    use crate::api::v1::{posts::All, Limit};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};
    use chrono::NaiveDate;

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/all")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = All::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tags() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/all")
            .add_query_params(&[("tag", "one two")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = All::builder().tags(vec!["one", "two"]).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tags_8() {
        let err = All::builder().tags(vec!["one"; 8]).build().unwrap_err();

        assert_eq!(
            &err.to_string(),
            "Endpoint only accepts up to 3 tags (received 8)"
        )
    }

    #[test]
    fn endpoint_start() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/all")
            .add_query_params(&[("start", "88")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = All::builder().start(88).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_results() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/all")
            .add_query_params(&[("results", "256")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = All::builder().results(256).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_fromdt() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/all")
            .add_query_params(&[("fromdt", "2001-05-06")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = All::builder()
            .fromdt(NaiveDate::from_ymd(2001, 5, 6))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_todt() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/all")
            .add_query_params(&[("todt", "2010-08-09")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = All::builder()
            .todt(NaiveDate::from_ymd(2010, 8, 9))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_meta() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/all")
            .add_query_params(&[("meta", "yes")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = All::builder().meta(true).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
        assert_eq!(All::secs_between_calls(), 300)
    }
}
