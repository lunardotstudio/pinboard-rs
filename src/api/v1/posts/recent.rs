// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;
use crate::api::v1::Limit;
use crate::api::endpoint_prelude::*;

/// Query the `update` endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option), build_fn(validate="Self::validate"))]
pub struct Recent<'a> {
    /// Tag filter (up to 3 tags)
    #[builder(setter(into),default)]
    tags: Option<Cow<'a, [&'a str]>>,
    /// Include a change detection signature in results
    #[builder(default)]
    count: Option<u8>,
}

impl<'a> Recent<'a> {
    /// Create a builder for the endpoint
    pub fn builder() -> RecentBuilder<'a> {
        RecentBuilder::default()
    }
}

impl<'a> RecentBuilder<'a> {
    // Check tags and count for constraints
    // There can only be 3 tags.
    // Count is limited to 100.
    fn validate(&self) -> Result<(), String> {
        if let Some(ref cow) =  self.tags {
            if let Some(xs) = cow {
                if xs.len() > 3 {
                    return Err(format!("Endpoint only accepts up to 3 tags (received {})", xs.len()))
                }
            }
        }

        if let Some(count) = self.count {
            if let Some(count) = count {
                if count > 100 {
                    return Err(format!("Endpoint only accepts `count` of 100 or less"))
                }
            }
        }

        Ok(())
    }
}

impl<'a> Endpoint for Recent<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/posts/recent".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("tag", self.tags.as_ref().map(|xs| xs.join(" ")))
            .push_opt("count", self.count);

        params
    }
}

impl<'a> Limit for Recent<'a> {
    fn secs_between_calls() -> usize {
	60
    }
}

#[cfg(test)]
mod tests {
    use crate::api::v1::{Limit,posts::Recent};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("v1/posts/recent").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint,"");

        let endpoint = Recent::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/recent")
            .add_query_params(&[("tag","Tag1 Tag2")])
            .build().unwrap();
        let client = SingleTestClient::new_raw(endpoint,"");

        let endpoint = Recent::builder().tags(vec!["Tag1","Tag2"]).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag_5() {
        let err = Recent::builder()
            .tags(vec!["Tag1","Tag2","Tag3","Tag4","Tag5"])
            .build().unwrap_err();
        assert_eq!(&err.to_string(), "Endpoint only accepts up to 3 tags (received 5)")
    }

    #[test]
    fn endpoint_count() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/recent")
            .add_query_params(&[("count","77")])
            .build().unwrap();
        let client = SingleTestClient::new_raw(endpoint,"");

        let endpoint = Recent::builder().count(77).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_count_201() {
        let err = Recent::builder()
            .count(201)
            .build().unwrap_err();
        assert_eq!(&err.to_string(), "Endpoint only accepts `count` of 100 or less")
    }

    #[test]
    fn limit() {
	assert_eq!(Recent::secs_between_calls(), 60)
    }
}
