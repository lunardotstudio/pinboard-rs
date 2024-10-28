// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Default {
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub id: String,
    pub hash: String,
    pub title: String,
    pub text: String,
    pub length: usize,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NoteSummary {
    pub id: String,
    pub hash: String,
    pub title: String,
    pub length: usize,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NoteList {
    pub count: usize,
    pub notes: Vec<NoteSummary>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    pub result_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostsDates {
    pub user: String,
    pub tag: String,
    dates: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostsList {
    pub tag: String,
    pub user: String,
    pub posts: Vec<Post>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostsRecent {
    pub user: String,
    pub date: DateTime<Utc>,
    pub posts: Posts,
}

pub type PostsSubbest = Vec<std::collections::HashMap<String, Vec<String>>>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostsUpdate {
    pub update_time: DateTime<Utc>,
}

pub type Tags = HashMap<String, usize>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserApiToken {
    #[serde(rename = "result")]
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserSecret {
    #[serde(rename = "result")]
    pub secret: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_recents() {
        let p = Post {
            href: "https://example.com".to_string(),
            description: "Description".to_string(),
            extended: "".to_string(),
            meta: "meta".to_string(),
            hash: "hash".to_string(),
            time: DateTime::from_timestamp(61, 0).unwrap(),
            shared: "no".to_string(),
            toread: "no".to_string(),
            tags: "tag".to_string(),
        };
        let pr = PostsRecent {
            date: DateTime::from_timestamp(61, 0).unwrap(),
            user: "User".to_string(),
            posts: vec![p],
        };

        assert_tokens(
            &pr,
            &[
                Token::Struct {
                    name: "PostsRecent",
                    len: 3,
                },
                Token::Str("user"),
                Token::Str("User"),
                Token::Str("date"),
                Token::Str("1970-01-01T00:01:01Z"),
                Token::Str("posts"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Post",
                    len: 9,
                },
                Token::Str("href"),
                Token::Str("https://example.com"),
                Token::Str("description"),
                Token::Str("Description"),
                Token::Str("extended"),
                Token::Str(""),
                Token::Str("meta"),
                Token::Str("meta"),
                Token::Str("hash"),
                Token::Str("hash"),
                Token::Str("time"),
                Token::Str("1970-01-01T00:01:01Z"),
                Token::Str("shared"),
                Token::Str("no"),
                Token::Str("toread"),
                Token::Str("no"),
                Token::Str("tags"),
                Token::Str("tag"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );

        let body = r#"{"date":"2024-10-27T17:38:11Z","user":"person","posts":[{"href":"https://some.web.site","description":"Words","extended":"More Words","meta":"Some meta thing","hash":"AAABBBCCCDDDEEEFFF000","time":"2024-01-01T00:00:00Z","shared":"no","toread":"no","tags":"one two"}]}"#;

        let val = serde_json::from_slice(body.as_bytes()).unwrap();
        let pst = serde_json::from_value::<PostsRecent>(val).unwrap();
        assert_eq!(pst.user, "person");
        assert_eq!(pst.posts.len(), 1);
        assert_eq!(pst.posts[0].href, "https://some.web.site");
    }
}
