// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Default {
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub id: String,
    pub hash: String,
    pub title: String,
    pub text: String,
    pub length: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NoteSummary {
    pub id: String,
    pub hash: String,
    pub title: String,
    pub length: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NoteList {
    pub count: i64,
    pub notes: Vec<NoteSummary>
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Post {
    pub href: String,
    pub description: String,
    pub extended: String,
    pub meta: String,
    pub hash: String,
    pub time: DateTime<Utc>,
    pub shared: String,
    pub toread: String,
    pub tags: String,
}

pub type Posts = Vec<Post>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostsAdd {
    pub result_code: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostsDates {
    pub user: String,
    pub tag: String,
    dates: HashMap<String, i64>
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostsList {
    pub date: DateTime<Utc>,
    pub user: String,
    pub posts: Vec<Post>,
}

pub type PostsSubbest = Vec<std::collections::HashMap<String,Vec<String>>>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostsUpdate {
    pub update_time: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserApiToken {
    #[serde(rename = "result")]
    pub token: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserSecret {
    #[serde(rename = "result")]
    pub secret: String
}
