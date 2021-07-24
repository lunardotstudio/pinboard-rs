// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;
use crate::api::v1::Limit;

/// Query the `list` endpoint.
#[derive(Debug, Clone, Copy, Builder)]
pub struct List {}

impl List {
    /// Create a builder for the endpoint
    pub fn builder() -> ListBuilder {
        ListBuilder::default()
    }
}

impl Endpoint for List {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/notes/list".into()
    }
}

impl Limit for List {}

#[cfg(test)]
mod tests {
    use crate::api::v1::Limit;
    use crate::api::v1::notes::List;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("v1/notes/list").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint,"");

        let endpoint = List::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
	assert_eq!(List::secs_between_calls(), 3)
    }
}
