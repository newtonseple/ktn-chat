#[macro_use]
extern crate serde_derive;

extern crate serde_json;

mod tcp_endpoint;

// The structs have small case fields for compatibility
mod request;
mod response;

pub use request::{RequestType, Request};

#[test]
fn it_works() {
}
