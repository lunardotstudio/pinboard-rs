// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use http::Uri;
use url::Url;

use crate::api::{ApiError, AsyncClient, Client};

pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}

/// A trait which represents a query for a Client
pub trait Query<T, C>
where
    C: Client,
{
    /// Perform the query against the Client
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// A trait which represents a query for an AsyncClient
#[async_trait]
pub trait AsyncQuery<T, C>
where
    C: AsyncClient,
{
    /// Perform the query against the AsyncClient
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
