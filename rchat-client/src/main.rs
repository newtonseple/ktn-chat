#![allow(dead_code)] // For developer sanity
extern crate rchat_common;
use std::io::BufRead;
use std::io;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::net::TcpStream;
use rchat_common::*;
mod console_io;
use console_io::*;


 struct Connection;
    impl TcpTransceive for Connection {
        type SendType = Request;
        type ReceiveType = Response;
    }

fn main() {
    let stream = TcpStream::connect("127.0.0.1:9999").expect("unable to connect");
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    Connection::run(stream, tx2, rx1);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        tx1.send(Request{request: RequestType::msg, content: Some("Hei på deg".to_string())});
        println!("\"Hei på deg\" sent");
    }
}