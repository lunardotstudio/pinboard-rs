// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use http::request::Builder as RequestBuilder;
use http::Response;
use url::Url;

use crate::api::ApiError;

/// A trait representing a REST-ful client
pub trait RestClient {
    /// The errors that may occur with this client
    type Error: Error + Send + Sync + 'static;

    /// Generate the URL for the endpoint for this client
    ///
    ///
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;
}

/// A trait representing a client which can communicate with Pinboard
pub trait Client: RestClient {
    /// Send a REST query
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client  which can communicate with Pinboard
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously.
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
