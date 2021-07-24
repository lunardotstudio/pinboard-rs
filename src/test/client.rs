// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::collections::HashMap;

use async_trait::async_trait;
use bytes::Bytes;
use derive_builder::Builder;
use http::request::Builder as RequestBuilder;
use http::{header, Method, Response, StatusCode};
use serde::ser::Serialize;
use thiserror::Error;
use url::Url;

use crate::api::{ApiError, AsyncClient, Client, RestClient};

#[derive(Debug, Builder)]
pub struct ExpectedUrl {
    #[builder(default = "Method::GET")]
    pub method: Method,
    pub endpoint: &'static str,
    #[builder(default)]
    pub query: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    #[builder(setter(strip_option, into), default)]
    pub content_type: Option<String>,
    #[builder(default)]
    pub body: Vec<u8>,
    #[builder(default = "StatusCode::OK")]
    pub status: StatusCode,
}

impl ExpectedUrlBuilder {
    pub fn add_query_params(&mut self, pairs: &[(&'static str, &'static str)]) -> &mut Self {
        self.query
            .get_or_insert_with(Vec::new)
            .extend(pairs.iter().cloned().map(|(k, v)| (k.into(), v.into())));
        self
    }

    pub fn body_str(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.bytes().collect());
        self
    }
}

impl ExpectedUrl {
    pub fn builder() -> ExpectedUrlBuilder {
        ExpectedUrlBuilder::default()
    }

    fn check(&self, method: Method, url: &Url) {
        // Test that the method is as expected.
        assert_eq!(method, self.method);
        // Ensure that the URL was not tampered with in the meantime.
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.username(), "");
        assert_eq!(url.password(), None);
        assert_eq!(url.host_str().unwrap(), "api.pinboard.invalid");
        assert_eq!(url.port(), None);
        assert_eq!(url.path(), format!("/{}", self.endpoint));
        println!("URL: {:?}", url);
        let mut count = 0;
        for (ref key, ref value) in url.query_pairs() {
            println!("CHECKING {}:{} against {:?}", key, value, self.query);
            let found = self.query.iter().any(|(expected_key, expected_value)| {
                key == expected_key && value == expected_value
            });

            if !found {
                panic!("unexpected query parameter `{}={}`", key, value);
            }
            count += 1;
        }
        assert_eq!(count, self.query.len());
        assert_eq!(url.fragment(), None);
    }
}

#[derive(Debug, Clone)]
struct MockResponse {
    status: StatusCode,
    data: Vec<u8>,
}

impl MockResponse {
    fn response(&self) -> Response<Vec<u8>> {
        Response::builder()
            .status(self.status)
            .body(self.data.clone())
            .unwrap()
    }
}

#[derive(Debug, Default)]
struct MockClient {
    response_map: HashMap<(Method, String), MockResponse>,
}

const CLIENT_STUB: &str = "https://api.pinboard.invalid";

pub struct SingleTestClient {
    client: MockClient,

    expected: ExpectedUrl,
}

impl SingleTestClient {
    pub fn new_raw<T>(expected: ExpectedUrl, data: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        let mut client = MockClient::default();

        let request = (expected.method.clone(), format!("/{}", expected.endpoint));
        let response = MockResponse {
            status: expected.status,
            data: data.into(),
        };

        client.response_map.insert(request, response);

        Self { client, expected }
    }

    pub fn new_json<T>(expected: ExpectedUrl, data: &T) -> Self
    where
        T: Serialize,
    {
        let data = serde_json::to_vec(data).unwrap();
        Self::new_raw(expected, data)
    }
}

#[derive(Debug, Error)]
#[error("test client error")]
pub enum TestClientError {}

impl RestClient for SingleTestClient {
    type Error = TestClientError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&format!("{}/{}", CLIENT_STUB, endpoint))?)
    }
}

impl Client for SingleTestClient {
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let url = Url::parse(&format!("{}", request.uri_ref().unwrap())).unwrap();
        self.expected
            .check(request.method_ref().unwrap().clone(), &url);
        assert_eq!(
            &body,
            &self.expected.body,
            "\nbody is not the same:\nactual  : {}\nexpected: {}\n",
            String::from_utf8_lossy(&body),
            String::from_utf8_lossy(&self.expected.body),
        );
        let headers = request.headers_ref().unwrap();
        let content_type = headers
            .get_all(header::CONTENT_TYPE)
            .iter()
            .map(|value| value.to_str().unwrap());
        if let Some(expected_content_type) = self.expected.content_type.as_ref() {
            itertools::assert_equal(content_type, [expected_content_type].iter().cloned());
        } else {
            assert_eq!(content_type.count(), 0);
        }

        let request = request.body(body).unwrap();

        Ok(self
            .client
            .response_map
            .get(&(request.method().clone(), request.uri().path().into()))
            .expect("no matching request found")
            .response()
            .map(Into::into))
    }
}

#[async_trait]
impl AsyncClient for SingleTestClient {
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        <Self as Client>::rest(self, request, body)
    }
}
