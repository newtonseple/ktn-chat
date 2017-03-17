#![feature(mpsc_select)]
#![allow(dead_code)] // For developer sanity

extern crate rchat_common;

use std::thread;

use std::net::{TcpListener, TcpStream};

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Select;

use rchat_common::{Request, Response};

fn main() {
    println!("Hello, world!");
    ChatServer::start();
    loop{}
}

struct ClientInfo {
    username: Option<String>,
    response_tx: Sender<Response>, // TODO: datatype
    request_rx: Receiver<Request>, // TODO: datatype
}

struct ChatServer {
    message_log: Vec<String>,
    clients: Vec<ClientInfo>,
    stream_rx: Receiver<TcpStream>,

}

impl ChatServer {
    pub fn start() {
        let mut chat_server = ChatServer{
            message_log: Vec::new(),
            clients: Vec::new(),
            stream_rx: ClientManager::get_stream_rx(),
        };
        loop {

            // Set up select
            let sel = Select::new();
            let mut stream_rx_handle = sel.handle(&chat_server.stream_rx); //++ 
            let mut request_handles = Vec::new();
            for client in chat_server.clients.iter() {
                request_handles.push((sel.handle(&client.request_rx)));
            }
            unsafe {
                stream_rx_handle.add(); //++
                for request_handle in request_handles.iter_mut(){
                    request_handle.add();
                }
            }
            let ret = sel.wait();

            // Handle select result
            if ret == stream_rx_handle.id() {
                let stream_result = stream_rx_handle.recv();
                println!("Got stream");
            } else {
                for (i, request_handle) in request_handles.iter_mut().enumerate(){
                    if ret == request_handle.id(){
                        println!("Got request from {}",i);
                        let request_result = request_handle.recv();
                        match request_result {
                            Ok(request) => {
                                println!("Request was: {}",i);
                            },
                            Err(e) => {
                                println!("Request was not received. {}",e);
                            },
                        }
                    }
                }
            }
        }
    }

    fn helpern() {
        // This is not necessarily necessary.
    }
}

struct ClientManager {
    tcp_listener: TcpListener,
    new_stream_tx: Sender<TcpStream>,
}

impl ClientManager {
    pub fn get_stream_rx() -> Receiver<TcpStream> {
        let (new_stream_tx, new_stream_rx) = channel(); 
        thread::Builder::new()
        .name("ClientManager".to_string())
        .spawn(move || {
            let client_manager = ClientManager{
                tcp_listener: TcpListener::bind("0.0.0.0:7777").expect("Could not bind TcpListener"),
                new_stream_tx
            };
            for stream in client_manager.tcp_listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("Got connection");
                        client_manager.new_stream_tx.send(stream).expect("new_stream_tx fail");
                    }
                    Err(_) => {
                        print!("Incoming connection not established");
                    }
                }
            }
        }).expect("Failed to start ClientManager");
        return new_stream_rx;
    }
}

struct ClientHandler {}

impl ClientHandler {
    pub fn handle_client() {
        loop {
            //listen to tcp, channel       
            //send to concerned
        }
    }
}