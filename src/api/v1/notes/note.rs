// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Query the `note` endpoint.
#[derive(Debug, Clone, Builder)]
pub struct Note<'a> {
    /// The note id
    #[builder(setter(into))]
    id: Cow<'a, str>
}

impl<'a> Note<'a> {
    /// Create a builder for the endpoint
    pub fn builder() -> NoteBuilder<'a> {
        NoteBuilder::default()
    }
}

impl<'a> Endpoint for Note<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v1/notes/note/{}",self.id).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::v1::notes::Note;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn id_is_required() {
        let err= Note::builder().build().unwrap_err();
        assert_eq!(&err.to_string(), "`id` must be initialized")
    }

    #[test]
    fn endpoint() {

        let endpoint = ExpectedUrl::builder().endpoint("v1/notes/note/IDHERE").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint,"");

        let endpoint = Note::builder().id("IDHERE").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
