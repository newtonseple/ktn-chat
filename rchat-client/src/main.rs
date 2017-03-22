#![allow(dead_code)] // For developer sanity
extern crate rchat_common;
use std::io::BufRead;
use std::io;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::net::TcpStream;
use rchat_common::*;
use std::thread;
use std::time::Duration;
mod console_io;
use console_io::{ConsolePrinter, ConsoleReader};


struct Connection;
impl TcpTransceive for Connection {
    type SendType = Request;
    type ReceiveType = Response;
}

fn main() {
    /*
    let stream = TcpStream::connect("127.0.0.1:9999").expect("unable to connect");
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    Connection::run(stream, tx2, rx1);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        tx1.send(Request::msg{ content: Some("Hei på deg".to_string())});
        println!("\"Hei på deg\" sent");
    }
    */
    let (tx, rx) = channel();
    let tx1 = tx.clone();
    thread::spawn(move || ConsoleReader::run(tx1));
    thread::spawn(move || ConsolePrinter::run(rx));
    loop {
        tx.send("HEI HEI".to_string()).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
}