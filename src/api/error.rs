// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::any;
use std::error::Error;

use thiserror::Error;

/// Errors which may occur when using API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// The client encountered an error
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },

    /// The URL failed to parse.
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },

    /// Body data could not be created
    #[error("failed to create form data: {}", source)]
    Body {
        /// the source of the error.
        #[from]
        source: BodyError,
    },

    /// JSON deserialization failed
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// the source of the error.
        #[from]
        source: serde_json::Error,
    },

    /// Pinboard returned an error without JSON information.
    #[error("pinboard internal server error {}", status)]
    PinboardService {
        /// The status code for the return.
        status: http::StatusCode,
        /// The error data from Pinboard.
        data: Vec<u8>,
    },

    
    /// Failed to parse and expected data type from JSON.
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The source of the error
        source: serde_json::Error,
        /// The name of the type that could not be deserialized
        typename: &'static str,
    },

    /// Pinboard returned an Error
    #[error("{}", msg)]
    Pinboard {
        /// The error message
        msg: String,
    },

    /// Pinboard returned an Error
    #[error("{}", obj)]
    PinboardObject {
        /// The error message
        obj: serde_json::Value,
    },

    /// Unrecognized Pinboard Error
    #[error("{}", obj)]
    PinboardUnrecognized {
        /// The full object from Pinboard
        obj: serde_json::Value
    }
}


/// Errors which may occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Body data could not be serialized from form parameters
    #[error("failed to URL encode form parameters: {}", source)]
    UrlEncoded {
        /// the source of the error
        #[from]
        source: serde_urlencoded::ser::Error,
    }
}


impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Generate an ApiError from a client
    pub fn client(source: E) -> Self {
        ApiError::Client {
            source
        }
    }

    pub(crate) fn server_error(status: http::StatusCode, body: &bytes::Bytes) -> Self {
        Self::PinboardService {
            status,
            data: body.into_iter().copied().collect(),
        }
    }
    
    pub(crate) fn from_pinboard(value: serde_json::Value) -> Self {
        // TODO: This is now how pinboard returns errors
        let error_message = value
            .pointer("/error_message");

        if let Some(error_message) = error_message {
            if let Some(msg) = error_message.as_str() {
                ApiError::Pinboard {
                    msg: msg.into(),
                }
            } else {
                ApiError::PinboardObject {
                    obj: error_message.clone(),
                }
            }
        } else {
            ApiError::PinboardUnrecognized {
                obj: value,
            }
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        ApiError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}
