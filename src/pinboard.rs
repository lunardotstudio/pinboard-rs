// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::convert::TryInto;
use std::fmt::{self, Debug};

use crate::auth::{Auth, AuthError};
use crate::api;
use bytes::Bytes;
use http::Response as HttpResponse;
use reqwest::blocking::Client;
use reqwest::Client as AsyncClient;
use thiserror::Error;
use url::Url;

#[derive(Debug,Error)]
#[non_exhaustive]
pub enum PinboardError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },

    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError
    }
}

type PinboardResult<T> = Result<T, PinboardError>;

/// A pinboard API for a single user
///
#[derive(Clone)]
pub struct Pinboard {
    /// The client to use for the API calls
    client: Client,
    /// The base URL to use for API calls
    url: Url,
    /// The authentication information to use for communication
    auth: Auth
}

impl Debug for Pinboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Pinboard")
            .field("url", &self.url)
            .finish()
    }
}

impl Pinboard {
    /// Create a new Pinboard API representation
    ///
    /// The `token` is the personal access token available on http://pinboard.in
    pub fn new<H, T>(host: H, token: T) -> PinboardResult<Self>
    where
        H: AsRef<str>,
        T: Into<String>,
    {
        Self::new_impl(
            host.as_ref(),
            Auth::Token(token.into())
        )
    }

    /// Internal method to create a new client
    fn new_impl(
        host: &str,
        auth: Auth
    ) -> PinboardResult<Self> {
        let url = Url::parse(&format!("https://{}/v2", host))?;
        let client = Client::new();
        let api = Pinboard {
            client,
            url,
            auth
        };

        Ok(api)
    }

    /// Create a new Pinboard API client builder.
    pub fn builder<H, T>(host: H, token: T) -> PinboardBuilder
    where
        H: Into<String>,
        T: Into<String>,
    {
        PinboardBuilder::new(host, token)
    }
}


pub struct PinboardBuilder {
    host: String,
    token: Auth,
}

impl PinboardBuilder {
    /// Create a new Pinboard API client builder.
    pub fn new<H, T>(host: H, token: T) -> Self
    where
        H: Into<String>,
        T: Into<String>,
    {
        Self {
            host: host.into(),
            token: Auth::Token(token.into()),
        }
    }

    pub fn build(&self) -> PinboardResult<Pinboard> {
        Pinboard::new_impl(
            &self.host,
            self.token.clone(),
        )
    }

    pub async fn build_async(&self) -> PinboardResult<AsyncPinboard> {
        AsyncPinboard::new_impl(
            &self.host,
            self.token.clone(),
        )
            .await
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },
    #[error("communication with pinboard: {}", source)]
    Communication{
        #[from]
        source: reqwest::Error,
    },
    #[error("`http` error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}

impl api::RestClient for Pinboard {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        let mut url = self.url.join(endpoint)?;
        self.auth.add_to_url(&mut url);
        url.query_pairs_mut().append_pair("format", "json");
        Ok(url)
    }
}

impl api::Client for Pinboard {
    fn rest(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<Self::Error>> {
        let call = || -> Result<_, RestError> {
            self.auth.set_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes()?)?)
        };
        call().map_err(api::ApiError::client)
    }
}

/// A representation of an asynchronous Pinboard API for a single user
///
#[derive(Clone)]
pub struct AsyncPinboard {
    /// The client to use for API calls
    client: reqwest::Client,
    /// The base to use for API calls
    url: Url,
    /// The authorization information to use for communication with Pinboard
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
    /// Internal method to create a new client
    async fn new_impl(
        host: &str,
        auth: Auth
    ) -> PinboardResult<Self> {
        let url = Url::parse(&format!("https://{}/v2", host))?;
        let client = AsyncClient::new();
        let api = AsyncPinboard {
            client,
            url,
            auth
        };

        Ok(api)
    }
}
