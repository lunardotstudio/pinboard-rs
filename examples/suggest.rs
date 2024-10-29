use pinboard_rs::api::{v1::posts::Suggest, Query};
use pinboard_rs::Pinboard;
use serde::Deserialize;
use std::env;
use url::Url;

#[derive(Deserialize)]
struct Popular {
    popular: Vec<String>,
}

#[derive(Deserialize)]
struct Recommended {
    recommended: Vec<String>,
}

#[derive(Deserialize)]
struct Results(Popular, Recommended);

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("USAGE: recents <TOKEN> <URL>")
    }

    let token = args.get(1).expect("Token required");
    let urlstr = args.get(2).expect("URL required");

    let url = Url::parse(urlstr).expect("Couldn't make URL");

    let suggest_endpoint = Suggest::builder()
        .url(url.clone())
        .build()
        .expect("building endpoint");
    let pb = Pinboard::new("api.pinboard.in", token).expect("Pinboard client");

    let res: Results = suggest_endpoint.query(&pb).unwrap();

    println!("Suggested tags for {}", url);
    println!("*Popular");
    for tag in res.0.popular {
        println!(" - {}", tag);
    }
    println!("*Recommended");
    for tag in res.1.recommended {
        println!(" - {}", tag);
    }
}
