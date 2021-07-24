// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use http::{HeaderMap, HeaderValue};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("header value error: {}", source)]
    HeaderValue {
        #[from]
        source: http::header::InvalidHeaderValue,
    },
}

/// A Pinboard API Token
///
#[derive(Clone)]
pub enum Auth {
    /// A personal access token, obtained online or through an api call
    Token(String),
}

type AuthResult<T> = Result<T, AuthError>;

impl Auth {
    /// Add the token to a request header
    ///
    pub fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> AuthResult<&'a mut HeaderMap<HeaderValue>> {
        match self {
            Auth::Token(token) => {
                let mut token_header_value = HeaderValue::from_str(&token)?;
                token_header_value.set_sensitive(true);
                headers.insert("X-Auth-Token", token_header_value);
            }
        }
        Ok(headers)
    }

    /// Add the token to a url
    pub fn add_to_url(&self, url: &mut url::Url) {
        match self {
            Auth::Token(token) => {
                let mut pairs = url.query_pairs_mut();
                pairs.append_pair("auth_token", token);
            }
        }
    }
}
