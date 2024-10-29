use chrono::NaiveDate;
use pinboard_rs::api::{v1::posts::Dates, Query};
use pinboard_rs::Pinboard;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize)]
struct AllResults {
    user: String,
    tag: String,
    dates: HashMap<String, usize>,
}

#[derive(Deserialize)]
struct JustDates {
    #[serde(with = "tuple_vec_map")]
    dates: Vec<(NaiveDate, usize)>,
}

// The way that serialization works means that each
// endpoint can return different concrete types.
// Each type just needs to be serializable from the
// json via serde.
//
// Here we have two strutures above. One takes all of
// returned results and stores it. The other just grabs
// the dates.
//
// In the code below, the same endoint is used to populate
// each structure. This means there is flexibility on the
// user side to develop any types and not feel required
// to use the built-in versions.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("USAGE: recents <TOKEN> [TAG]*")
    }

    let token = args.get(1).expect("Token required");
    let tags: Vec<_> = args[2..].iter().map(AsRef::<str>::as_ref).collect();

    let dates_endpoint = Dates::builder()
        .tags(tags)
        .build()
        .expect("building endpoint");
    let pb = Pinboard::new("api.pinboard.in", token).expect("Pinboard client");

    let res: AllResults = dates_endpoint.query(&pb).unwrap();
    println!("`AllResults` Struture");
    println!("user: {}", res.user);
    if res.tag != "" {
        println!("tags: {}", res.tag);
    }
    println!("Dates: (Date :: Count)");
    for (date, count) in res.dates {
        println!(" * {} :: {}", date, count);
    }

    let res: JustDates = dates_endpoint.query(&pb).unwrap();
    println!("`JustDates` Struture");
    for (date, count) in res.dates {
        println!(" * {} :: {}", date, count);
    }
}
