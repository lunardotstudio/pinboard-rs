// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::convert::TryInto;
use std::fmt::{self, Debug};

use crate::api;
use crate::auth::Auth;
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::TryFutureExt;
use http::Response as HttpResponse;
use reqwest::Client as AsyncClient;
use url::Url;

use crate::pinboard::{PinboardResult, RestError};

/// A representation of an asynchronous Pinboard API for a single user
///
#[derive(Clone)]
pub struct AsyncPinboard {
    /// The client to use for API calls
    client: reqwest::Client,
    /// The base URL for API calls
    url: Url,
    /// The authorization for communication with Pinboard
    auth: Auth,
}

impl Debug for AsyncPinboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AsyncPinboard")
            .field("url", &self.url)
            .finish()
    }
}

impl AsyncPinboard {
    /// Create a new async Pinboard API representation
    ///
    /// The `token` is the personal access token available at <https://pinboard.in>
    pub async fn new<H, T>(host: H, token: T) -> PinboardResult<Self>
    where
        H: AsRef<str>,
        T: Into<String>,
    {
        Self::new_impl(host.as_ref(), Auth::Token(token.into())).await
    }

    /// Internal method to create a new client
    async fn new_impl(host: &str, auth: Auth) -> PinboardResult<Self> {
        let url = Url::parse(&format!("https://{}/", host))?;
        let client = AsyncClient::new();
        let api = AsyncPinboard { client, url, auth };

        Ok(api)
    }
}

#[async_trait]
impl api::AsyncClient for AsyncPinboard {
    /// Perform async reqwest query
    async fn rest_async(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<Self::Error>> {
        let call = || async {
            self.auth.set_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().map_err(api::ApiError::client).await
    }
}

impl api::RestClient for AsyncPinboard {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        let mut url = self.url.join(endpoint)?;
        self.auth.add_to_url(&mut url);
        url.query_pairs_mut().append_pair("format", "json");
        Ok(url)
    }
}
