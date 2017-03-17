extern crate serde;
extern crate serde_json;

use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::marker::Send;


pub struct Connection;

/// Trait for nonblocking TCP connection.
/// To implement this trait for ´Connection´
/// spesify ´SendType´ and ´ReceiveType´ to get ´run´.
pub trait Transceive {
    type SendType: 'static + Serialize + Send;
    type ReceiveType: 'static + Deserialize + Send;


    fn run(stream: TcpStream, recieve_tx: Sender<Self::ReceiveType>, send_rx: Receiver<Self::SendType>) {
        let sender = TcpSender::new(stream.try_clone().expect("Unable to clone TcpStream"), send_rx);
        let reciver = TcpReciever::new(stream.try_clone().expect("Unable to clone TcpStream"), recieve_tx);
        
        thread::spawn(move || {
            sender.run();
        });

        thread::spawn(move || {
            reciver.run();
        });
    }
} 



struct TcpReciever<R> where R: Deserialize {
    stream: TcpStream,
    tx: Sender<R>,
}


impl<R> TcpReciever<R> where R: Deserialize {    
    pub fn new(stream: TcpStream, tx: Sender<R>) -> Self {
        TcpReciever{stream, tx}
    }

    pub fn run(mut self) -> ! {
        loop {
            let mut recieved = String::new();
            self.stream.read_to_string(&mut recieved).expect("Unable to read from Tcpstream");
            let recieved: R = serde_json::from_str(&recieved).expect("Unable to deserialize");
            self.tx.send(recieved).expect("Unable to send received object");
        }
    }
}


struct TcpSender<T> where T: Serialize {
    stream: TcpStream,
    rx: Receiver<T>,
}

impl<T> TcpSender<T> where T: Serialize {
     pub fn new(stream: TcpStream, rx: Receiver<T>) -> Self {
         TcpSender{stream, rx}
     }

     pub fn run(mut self) -> ! {
         loop {
             let to_send = self.rx.recv().expect("Unable to receive something to send");
             let to_send = serde_json::to_string(&to_send).expect("Unable to serialize");
             self.stream.write(to_send.as_bytes()).expect("Unable to send object");
         }
     }
}
