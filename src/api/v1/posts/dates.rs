// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Query the `update` endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option), build_fn(validate="Self::validate"))]
pub struct Dates<'a> {
    /// Tag filter (up to 3 tags)
    #[builder(setter(into),default)]
    tags: Option<Cow<'a, [&'a str]>>,
}

impl<'a> Dates<'a> {
    /// Create a builder for the endpoint
    pub fn builder() -> DatesBuilder<'a> {
        DatesBuilder::default()
    }
}

impl<'a> DatesBuilder<'a> {
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

        Ok(())
    }
}

impl<'a> Endpoint for Dates<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/posts/dates".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("tag", self.tags.as_ref().map(|xs| xs.join(" ")));
        params
    }
}

#[cfg(test)]
mod tests {
    use crate::api::v1::posts::Dates;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("v1/posts/dates").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint,"");

        let endpoint = Dates::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/dates")
            .add_query_params(&[("tag","Tag1 Tag2")])
            .build().unwrap();
        let client = SingleTestClient::new_raw(endpoint,"");

        let endpoint = Dates::builder().tags(vec!["Tag1","Tag2"]).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag_4() {
        let err = Dates::builder()
            .tags(vec!["Tag1","Tag2","Tag3","Tag4"])
            .build().unwrap_err();
        assert_eq!(&err.to_string(), "Endpoint only accepts up to 3 tags (received 4)")
    }
}
