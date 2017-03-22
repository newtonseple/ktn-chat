#![feature(mpsc_select)]
#![allow(dead_code)] // For developer sanity

extern crate rchat_common;

use std::thread;

use std::net::{TcpListener, TcpStream};

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Select;

use rchat_common::{Request, Response, TcpTransceive};

fn main() {
    println!("Hello, world!");
    ChatServer::start();
    loop{}
}

struct ClientInfo {
    username: Option<String>,
    response_tx: Sender<Response>,
    request_rx: Receiver<Request>,
}

enum ChatServerAction {
    DoNothing,
    AddClient(ClientInfo),
    RemoveClient(usize),
    HandleRequest(Request, usize), //request and who it's from
}

struct ChatServer {
    //States
    message_log: Vec<String>,
    clients: Vec<ClientInfo>,
    stream_rx: Receiver<TcpStream>,
    action: ChatServerAction,
}

impl ChatServer {
    pub fn start() {
        let mut chat_server = ChatServer{
            message_log: Vec::new(),
            clients: Vec::new(),
            stream_rx: ClientManager::get_stream_rx(),
            action: ChatServerAction::DoNothing,
        };
        loop {
            { // Scope for select. Cannot change clients inside!

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
                    match stream_result {
                        Ok(stream) => {
                            let (request_tx, request_rx) = channel(); 
                            let (response_tx, response_rx) = channel();
                            ServerTcpTransceiver::run(stream, request_tx, response_rx);
                            chat_server.action = ChatServerAction::AddClient(ClientInfo{
                                username: None,
                                response_tx,
                                request_rx,
                            })
                        },
                        Err(e) => {
                            println!("Did not get stream. {}",e);
                        },
                    }
                } else {
                    for (i, request_handle) in request_handles.iter_mut().enumerate(){
                        if ret == request_handle.id(){
                            println!("Got request from {}",i);
                            let request_result = request_handle.recv();
                            match request_result {
                                Ok(request) => {
                                    println!("Request from {:?} was: {:?}",i,request);
                                    chat_server.action = ChatServerAction::HandleRequest(request, i);
                                },
                                Err(e) => {
                                    println!("Request was not received. {:?}",e);
                                    chat_server.action = ChatServerAction::RemoveClient(i);
                                },
                            }
                        }
                    }
                }
            }
            match chat_server.action{
                    ChatServerAction::DoNothing => {
                        unreachable!();
                    },
                    ChatServerAction::AddClient(client) => {
                        chat_server.clients.push(client);
                    },
                    ChatServerAction::RemoveClient(i) => {
                        chat_server.clients.swap_remove(i);
                    },
                    ChatServerAction::HandleRequest(request, i) => {
                        match request{
                            Request::login(content) => {},
                            Request::logout(content) => {},
                            Request::msg(content) => {},
                            Request::names(content) => {},
                            Request::help(content) => {},
                        }
                    },
            }
            chat_server.action = ChatServerAction::DoNothing;
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

struct ServerTcpTransceiver;
impl TcpTransceive for ServerTcpTransceiver {
    type SendType = Response;
    type ReceiveType = Request;
}