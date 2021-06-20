// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Query the `v1/tags/delete` endpoint.
#[derive(Debug, Clone, Builder)]
pub struct Delete<'a> {
    /// The bookmark to delete
    #[builder(setter(into))]
    tag: Cow<'a, str>
}

impl<'a> Delete<'a> {
    /// Create a builder for the endpoint
    pub fn builder() -> DeleteBuilder<'a> {
        DeleteBuilder::default()
    }
}

impl<'a> Endpoint for Delete<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/tags/delete".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("tag", self.tag.as_ref());
        params
    }
}

#[cfg(test)]
mod tests {
    use crate::api::v1::tags::Delete;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn tag_is_required() {
        let err = Delete::builder().build().unwrap_err();
        assert_eq!(&err.to_string(),"`tag` must be initialized")
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/tags/delete")
            .add_query_params(&[("tag", "buh-bye")])
            .build().unwrap();
        let client = SingleTestClient::new_raw(endpoint,"");

        let endpoint = Delete::builder().tag("buh-bye").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
