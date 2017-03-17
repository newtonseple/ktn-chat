#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod connection;

// The structs have small case fields for compatibility
mod request;
mod response;

pub use request::{RequestType, Request};
pub use response::Response;
pub use connection::{Connection, Tranceive};

#[test]
fn it_works() {
}
