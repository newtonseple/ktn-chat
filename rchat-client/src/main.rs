#![feature(mpsc_select)]

extern crate rchat_common;

use std::sync::mpsc::channel;
use std::net::TcpStream;
use rchat_common::*;
use std::thread;
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
    let ip = args.nth(1).expect("Please provide IP of the server as the 1st argument");
    let port: u16 = args.nth(0).unwrap().parse().expect("Please provide port number of the server as the 2nd argument");
    let stream = TcpStream::connect((ip.as_str(), port)).expect("unable to connect");

    println!("Login to start chatting");
    println!("Give commands on the form <Command> [Argument]");
    println!("Print \"help <ENTER>\" to begin");

    // Set up necessary threads for async I/O
    let (server_output_tx, server_output_rx) = channel();
    let (server_input_tx, server_input_rx) = channel();
    let (user_output_tx, user_output_rx) = channel();
    let (user_input_tx, user_input_rx) = channel();
    thread::Builder::new().name("User_input_thread".to_string()).spawn(move || {
        ConsoleReader::run(user_input_tx);
    }).expect("Unable to create user_input_thread");

    thread::Builder::new().name("User_output_thread".to_string()).spawn(move || {
        ConsolePrinter::run(user_output_rx);
    }).expect("unable to create user_output_thread");

    Connection::run(stream, server_input_tx, server_output_rx);

    loop {
        select! {
            request = user_input_rx.recv() => { 
                if let Ok(request) = request.expect("Unable to receive user input").parse() {
                    server_output_tx.send(request).expect("Unable to send to TCPTransmit");
                } else {
                    user_output_tx.send("Please give a valid input".to_string())
                        .expect("Unable to send to  ConsolePrinter");
                }
            },
            response = server_input_rx.recv() => {
                user_output_tx.send(format!("{}", response.expect("Unable to receive server responses")))
                    .expect("Unable to print to user");
            }
        }
    }
}