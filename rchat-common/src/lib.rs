#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod connection;

// The structs have small case fields for compatibility
mod request;
mod response;

pub use request::Request;
pub use response::Response;
pub use connection::TcpTransceive;

#[test]
fn it_works() {
}
