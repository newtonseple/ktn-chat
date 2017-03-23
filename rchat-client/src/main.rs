extern crate rchat_common;
use std::io::BufRead;
use std::io;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::net::TcpStream;
use rchat_common::*;
use std::thread;
use std::time::Duration;
use std::env::args;

mod console_io;
use console_io::{ConsolePrinter, ConsoleReader};

struct Connection;
impl TcpTransceive for Connection {
    type SendType = Request;
    type ReceiveType = Response;
}

fn main() {
    let mut args = args();
    if let Some(ip) = args.nth(1) {} else {panic!("Please provide the IP of the server as the first argument");};
    if let Some(port) = args.nth(2) {} else {panic!("Please provide the port of the server as the first argument");};
    let stream = TcpStream::connect((ip, port)).expect("unable to connect");

    println!("Login to start chatting");
    println!("Give commands on the form <Command> [Argument]");
    println!("Print \"help <ENTER>\" to begin");
    /*
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    Connection::run(stream, tx2, rx1);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        tx1.send(Request::msg{ content: Some("Hei på deg".to_string())});
        println!("\"Hei på deg\" sent");
    }
    */
    /*
    let (tx, rx) = channel();
    let tx1 = tx.clone();
    thread::spawn(move || ConsoleReader::run(tx1));
    thread::spawn(move || ConsolePrinter::run(rx));
    loop {
        tx.send("HEI HEI".to_string()).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
    */

    // Set up necessary threads for async I/O
    let (user_input_tx, user_input_rx) = channel();
    thread::Builder::new().name("User_input_thread".to_string()).spawn(move || {
        ConsoleReader::run(user_input_tx);
    }).expect("Unable to create user_input_thread");

    let (user_output_tx, user_output_rx) = channel();
    thread::Builder::new().name("User_output_thread".to_string()).spawn(move || {
        ConsolePrinter::run(user_output_rx);
    }).expect("unable to create user_output_thread");

    let (server_input_tx, server_input_rx) = channel();
    let (server_output_tx, server_output_rx) = channel();
    Connection::run(stream, server_input_tx, server_output_rx);



}