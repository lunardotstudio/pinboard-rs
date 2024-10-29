# Pinboard API

This library provides an interface to communicate with the
[Pinboard.in](https://pinboard.in) bookmarking service.

The endpoint structures are organized into two parts representing the
[V1](https://pinboard.in/api/) and
[V2](https://pinboard.in/api/v2/overview) versions of the API. Please
note that at this time, the V2 API is not live.

This library has synchronous and asychronous clients. The latter is available
with the `async` feature.

## Installation

Install pinboard-rs via cargo. The default installation includes the
`async` feature.

```bash
 cargo add pinboard-rs
```

## Usage/Examples

This library approaches API interaction in a different way. Each
endpoint connection starts with a builder to put together the
parameters and data required to query the endpoint.

The client that does the querying is created as a separate object and
then passed to the query function of the endpoint.

```rust,no_run
use pinboard_rs::api::{v1::posts::Recent, Query};
use pinboard_rs::types::v1::PostsRecent;
use pinboard_rs::Pinboard;

fn main() {
    /// Set the parameters to be used in the builder
    let token = "<TOKEN>";
    let x     = 5;
    let tags  = vec!["tags","for","filtering"];

    // Build the endpoint with the necessary parameters
    let recent_endpoint = Recent::builder()
        .count(x)
        .tags(&tags)
        .build()
        .expect("building endpoint");
        
    // Create a client to query the endpoints
    let pb = Pinboard::new("api.pinboard.in", token).expect("Pinboard client");

    // Query the endpoint and store the results
    let recent_posts: PostsRecent = recent_endpoint.query(&pb).unwrap();
    
    // Print out the results (debug view of structure)
    println!("Recent posts: {:?}", recent_posts)
}
```

The data returned through the query of the endoint is derserialized
through [serde](https://serde.rs/), meaning that the query method can
be used with any data structure that can deserialize the returned json.

Additional examples are available to run in the [examples
directory](examples/).  Run them with cargo:

```bash
 cargo run --example recent
```

## Running Tests

This library tests all endpoints and builders extensively. Run the
tests with cargo as follows:

```bash
 cargo test
```

## Acknowledgements

The structure for this library is based heavily on the work of Ben
Boeckel and the [gitlab crate](https://crates.io/crates/gitlab).

Read Ben's article on [Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is).

## License

Licensed under [MIT](LICENSE-MIT.txt) or [Apache 2.0](LICENSE-APACHE.txt) at your discretion.

Copyright © 2021-2024, Lunar Studio
