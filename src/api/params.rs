// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;

use chrono::{DateTime, NaiveDate, Utc};
use url::Url;

/// A trait representing a parameter value
pub trait ParamValue<'a> {
    #[allow(clippy::wrong_self_convention)]
    /// The parameter value as a string
    fn as_value(&self) -> Cow<'a, str>;
}

impl ParamValue<'static> for bool {
    fn as_value(&self) -> Cow<'static, str> {
        if *self {
            "true".into()
        } else {
            "false".into()
        }
    }
}

impl<'a> ParamValue<'a> for &'a str {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl ParamValue<'static> for String {
    fn as_value(&self) -> Cow<'static, str> {
        self.clone().into()
    }
}

impl<'a> ParamValue<'a> for &'a String {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl<'a> ParamValue<'a> for Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        self.clone()
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).clone()
    }
}

impl ParamValue<'static> for u8 {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self).into()
    }
}

impl ParamValue<'static> for u64 {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self).into()
    }
}

impl ParamValue<'static> for f64 {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self).into()
    }
}

impl ParamValue<'static> for DateTime<Utc> {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            .into()
    }
}

impl ParamValue<'static> for NaiveDate {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self.format("%Y-%m-%d")).into()
    }
}

impl ParamValue<'static> for &url::Url {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self).into()
    }
}

/// A structure for query parameters.
#[derive(Debug, Default, Clone)]
pub struct QueryParams<'a> {
    params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> QueryParams<'a> {
    /// Push a single parameter.
    pub fn push<'b, K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params.push((key.into(), value.as_value()));
        self
    }

    /// Push a single parameter.
    pub fn push_opt<'b, K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        if let Some(value) = value {
            self.params.push((key.into(), value.as_value()));
        }
        self
    }

    /// Push a set of parameters.
    pub fn extend<'b, I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params
            .extend(iter.map(|(key, value)| (key.into(), value.as_value())));
        self
    }

    /// Add the parameters to a URL.
    pub fn add_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        pairs.extend_pairs(self.params.iter());
    }
}

#[cfg(test)]
mod tests {
    use crate::api::ParamValue;

    #[test]
    fn bool_str() {
        let items = &[(true, "true"), (false, "false")];

        for (i, s) in items {
            assert_eq!((*i).as_value(), *s);
        }
    }
}
