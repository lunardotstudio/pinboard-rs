// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::endpoint_prelude::*;
use crate::api::v1::Limit;
use chrono::NaiveDate;
use derive_builder::Builder;

/// Query the `update` endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct Add<'a> {
    /// The bookmark to save
    url: url::Url,
    /// The title of the link (backwards-compatible name)
    #[builder(setter(into), default = "self.default_description()?.into()")]
    description: Cow<'a, str>,
    /// The description of the link (backwards-compatible name)
    #[builder(setter(into), default)]
    extended: Option<Cow<'a, str>>,
    /// The tags to add (limit of 100)
    #[builder(setter(into), default)]
    tags: Option<Cow<'a, [&'a str]>>,
    /// Creation time of this bookmark
    #[builder(default)]
    dt: Option<NaiveDate>,
    /// Whether or not to replace the current bookmark (server default is yes)
    /// An error is thrown when this is tru and a bookmark already exists for the url
    #[builder(default)]
    replace: Option<bool>,
    /// Make the bookmark public
    #[builder(default)]
    shared: Option<bool>,
    /// Marks the bookmark as unread.
    #[builder(default)]
    toread: Option<bool>,
}

impl<'a> AddBuilder<'a> {
    // Ensure there is something for a default description
    fn default_description(&self) -> Result<String, String> {
        match self.url {
            Some(ref url) => Ok(url.to_string()),
            _ => Err("Could not make default `description` from `url`".to_string()),
        }
    }

    // Check that the tags to not exceed 100
    fn validate(&self) -> Result<(), String> {
        if let Some(Some(ref xs)) = self.tags {
            if xs.len() > 100 {
                return Err(format!(
                    "Endpoint only accepts up to 100 tags (received {})",
                    xs.len()
                ));
            }
        }
        Ok(())
    }
}

impl<'a> Add<'a> {
    /// Create a builder for the endpoint
    pub fn builder() -> AddBuilder<'a> {
        AddBuilder::default()
    }
}

impl<'a> Endpoint for Add<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v1/posts/add".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push("url", self.url.as_ref())
            .push("description", self.description.as_ref())
            .push_opt("extended", self.extended.as_ref())
            .push_opt("tags", self.tags.as_ref().map(|xs| xs.join(" ")))
            .push_opt("dt", self.dt)
            .push_opt(
                "replace",
                self.replace.map(|x| if x { "yes" } else { "no" }),
            )
            .push_opt("shared", self.shared.map(|x| if x { "yes" } else { "no" }))
            .push_opt("toread", self.toread.map(|x| if x { "yes" } else { "no" }));

        params
    }
}

impl<'a> Limit for Add<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::v1::{posts::Add, Limit};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};
    use chrono::NaiveDate;

    static TITLE: &str = "Some Title";
    static URL: &str = "http://pinboard.test/";
    fn test_url() -> url::Url {
        url::Url::parse(URL).unwrap()
    }

    #[test]
    fn url_is_required() {
        let err = Add::builder().description(TITLE).build().unwrap_err();
        assert_eq!(&err.to_string(), "`url` must be initialized")
    }

    #[test]
    fn description_is_required() {
        let add = Add::builder().url(test_url()).build().unwrap();
        assert_eq!(add.description, URL)
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/add")
            .add_query_params(&[("url", URL), ("description", TITLE)])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Add::builder()
            .url(test_url())
            .description(TITLE)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_extended() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/add")
            .add_query_params(&[
                ("url", URL),
                ("description", TITLE),
                ("extended", "some extended text"),
            ])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Add::builder()
            .url(test_url())
            .description(TITLE)
            .extended("some extended text")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tags() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/add")
            .add_query_params(&[("url", URL), ("description", TITLE), ("tags", "one two")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Add::builder()
            .url(test_url())
            .description(TITLE)
            .tags(vec!["one", "two"])
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tags_101() {
        let err = Add::builder()
            .url(test_url())
            .description(TITLE)
            .tags(vec!["one"; 101])
            .build()
            .unwrap_err();

        assert_eq!(
            &err.to_string(),
            "Endpoint only accepts up to 100 tags (received 101)"
        )
    }

    #[test]
    fn endpoint_dt() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/add")
            .add_query_params(&[("url", URL), ("description", TITLE), ("dt", "2021-03-04")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Add::builder()
            .url(test_url())
            .description(TITLE)
            .dt(NaiveDate::from_ymd_opt(2021, 3, 4).expect("Valid date"))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_replace() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/add")
            .add_query_params(&[("url", URL), ("description", TITLE), ("replace", "yes")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Add::builder()
            .url(test_url())
            .description(TITLE)
            .replace(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/add")
            .add_query_params(&[("url", URL), ("description", TITLE), ("shared", "yes")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Add::builder()
            .url(test_url())
            .description(TITLE)
            .shared(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_toread() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("v1/posts/add")
            .add_query_params(&[("url", URL), ("description", TITLE), ("toread", "yes")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Add::builder()
            .url(test_url())
            .description(TITLE)
            .toread(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn limit() {
        assert_eq!(Add::secs_between_calls(), 3)
    }
}
