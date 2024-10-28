use pinboard_rs::api::{v1::posts::Recent, Query};
use pinboard_rs::types::v1::PostsRecent;
use pinboard_rs::Pinboard;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("USAGE: recents <TOKEN> [count] [tag]*")
    }

    let token = args.get(1).expect("Token required");

    let x: u8 = args
        .get(2)
        .map(|c_str| c_str.parse::<u8>().unwrap_or(10))
        .unwrap_or(10);

    let tags: Vec<_> = args[3..].iter().map(AsRef::<str>::as_ref).collect();

    let recent_endpoint = Recent::builder()
        .count(x)
        .tags(&tags)
        .build()
        .expect("building endpoint");
    let pb = Pinboard::new("api.pinboard.in", token).expect("Pinboard client");

    let rs: PostsRecent = recent_endpoint.query(&pb).unwrap();

    println!("{} Recent posts for {} at {}", x, rs.user, rs.date);
    for p in rs.posts {
        println!("- {}\n  {}", p.description, p.href);
        if p.tags.len() > 0 {
            let tgs: Vec<_> = p.tags.split(" ").collect();
            let tstr = tgs.join(", ");
            println!("  ({})", tstr);
        }
    }
}
