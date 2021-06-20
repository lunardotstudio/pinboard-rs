# Pinboard API

This library provides an interface to communicate with the
[Pinboard.in](https://pinboard.in) bookmarking service.

The endpoint structures are organized into two parts representing the
[V1](https://pinboard.in/api/) and
[V2](https://pinboard.in/api/v2/overview) versions of the API. Please
note that at this time, the V2 API is not live.

# Inspiration

The structure for this library is based heavily on the work of Ben
Boeckel and the [gitlab crate](https://crates.io/crates/gitlab).

Read Ben's article on [Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is).
