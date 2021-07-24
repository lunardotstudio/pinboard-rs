// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;
use crate::api::v1::Limit;
use derive_builder::Builder;

/// Query the `v1/tags/rename` endpoint.
#[derive(Debug, Clone, Builder)]
pub struct Rename<'a> {
    /// The old tag name
    #[builder(setter(into))]
    old: Cow<'a, str>,
    /// The new tag name
    #[builder(setter(into))]
    new: Cow<'a, str>,
}

impl<'a> Rename<'a> {
    /// Create a builder for the endpoint
    pub fn builder() -> RenameBuilder<'a> {
        RenameBuilder::default()
    }
}

impl<'a> Endpoint for Rename<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/tags/rename".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push("old", self.old.as_ref())
            .push("new", self.new.as_ref());
        params
    }
}

impl<'a> Limit for Rename<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::v1::{tags::Rename, Limit};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn old_is_required() {
        let err = Rename::builder().build().unwrap_err();
        assert_eq!(&err.to_string(), "`old` must be initialized")
    }

    #[test]
    fn new_is_required() {
        let err = Rename::builder().old("old").build().unwrap_err();
        assert_eq!(&err.to_string(), "`new` must be initialized")
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/tags/rename")
            .add_query_params(&[("old", "buh-bye"), ("new", "see-ya")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Rename::builder()
            .old("buh-bye")
            .new("see-ya")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
        assert_eq!(Rename::secs_between_calls(), 3)
    }
}
