#![feature(mpsc_select)]
#![allow(dead_code)] // For developer sanity

extern crate rchat_common;
extern crate regex;
extern crate chrono;

use std::thread;

use std::net::{TcpListener, TcpStream};

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Select;

use regex::Regex;

use rchat_common::{Request, Response, TcpTransceive};

fn main() {
    println!("Hello, world!");
    ChatServer::start();
    loop{}
}

//trait TestTrait {}

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
    message_log: Vec<Response>,
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
                                    //let timestamp = "TIMESTAMP".to_string();
                                    let now = chrono::UTC::now();
                                    let timestamp =  now.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                                    let response = Response::error{timestamp, sender: "SERVER".to_string(), content: "Invalid packet format. Connection dropped.".to_string()};
                                    chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                                    chat_server.action = ChatServerAction::RemoveClient(i);
                                },
                            }
                        }
                    }
                }
            }
            match chat_server.action{
                    ChatServerAction::DoNothing => {
                        //unreachable!();
                    },
                    ChatServerAction::AddClient(client) => {
                        chat_server.clients.push(client);
                    },
                    ChatServerAction::RemoveClient(i) => {
                        chat_server.clients.swap_remove(i);
                    },
                    ChatServerAction::HandleRequest(request, i) => {
                        let now = chrono::UTC::now();
                        let timestamp =  now.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        //let timestamp = "TIMESTAMP".to_string();
                        match request{
                            Request::login{content: Some(username)} => {
                                let re = Regex::new(r"^[a-zA-Z0-9]*$").unwrap();
                                if ChatServer::get_names(&(chat_server.clients)).contains(&username) || !re.is_match(username.as_str()){
                                    println!("client {} tried login as {}",i,username);
                                    let response = Response::error{timestamp, sender: "SERVER".to_string(), content: "Name taken or invalid".to_string()};
                                    chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                                } else {
                                    println!("client {} logged in as {}",i,username);
                                    chat_server.clients.get_mut(i).unwrap().username = Some(username);
                                    let response = Response::info{timestamp: timestamp.clone(), sender: "SERVER".to_string(), content: "Logged in".to_string()};
                                    chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                                    let response = Response::history{timestamp, sender: "SERVER".to_string(), content: chat_server.message_log.clone()};
                                    chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                                }
                            },
                            Request::logout{content: _} => {
                                let mut logout = false;
                                if let Some(_) = chat_server.clients.get(i).unwrap().username{ //wtf
                                    println!("client {} logout",i);
                                    logout = true;
                                } else {
                                    let response = Response::error{timestamp: timestamp.clone(), sender: "SERVER".to_string(), content: "Not logged in".to_string()};
                                    chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                                }
                                if logout{
                                    chat_server.clients.get_mut(i).unwrap().username = None;
                                    let response = Response::info{timestamp: timestamp.clone(), sender: "SERVER".to_string(), content: "Logged out".to_string()};
                                    chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                                }
                            },
                            Request::msg{content: Some(message)} => {
                                println!("client {} msg {}",i,message);       
                                if let Some(ref username) = chat_server.clients.get(i).unwrap().username{ //wtf
                                    let response = Response::message{timestamp, sender: username.clone(), content: message.clone()};
                                    for client in chat_server.clients.iter(){
                                        client.response_tx.send(response.clone()).expect("Could not send");
                                    }
                                    chat_server.message_log.push(response);
                                } else {
                                    let response = Response::error{timestamp, sender: "SERVER".to_string(), content: "Not logged in".to_string()};
                                    chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                                }

                            },
                            Request::names{content: _} => {
                                println!("client {} tried to get names",i);
                                if let Some(_) = chat_server.clients.get(i).unwrap().username{ //wtf
                                    let names = "Names: ".to_string() + &ChatServer::get_names(&(chat_server.clients)).join(", ");
                                    let response = Response::info{timestamp, sender: "SERVER".to_string(), content: names};
                                    chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                                } else {
                                    let response = Response::error{timestamp, sender: "SERVER".to_string(), content: "Not logged in".to_string()};
                                    chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                                }
                            },
                            Request::help{content: _} => {
                                println!("client {} tried to get help",i);
                                let response = Response::info{timestamp, sender: "SERVER".to_string(), content: "Valid requests: login <username>, logout, msg <message>, names, help".to_string()};
                                chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                            },
                            _ => {
                                println!("I don't know what client {} tried",i);
                                let response = Response::error{timestamp, sender: "SERVER".to_string(), content: "Invalid request".to_string()};
                                chat_server.clients.get(i).unwrap().response_tx.send(response).expect("Send failed");
                            }
                        }
                    },
            }
            chat_server.action = ChatServerAction::DoNothing;
        }
    }

    fn get_names(clients: &Vec<ClientInfo>) -> Vec<String> {
        // This is not necessarily necessary.
        let mut names = Vec::new();
            for ref client in clients.iter(){
                if let Some(ref username) = client.username {
                    names.push(username.clone());
                }
            }
        names
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
                tcp_listener: TcpListener::bind("0.0.0.0:9999").expect("Could not bind TcpListener"),
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