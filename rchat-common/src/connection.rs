
extern crate serde;
extern crate serde_json;

use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::marker::Send;

/// Trait for nonblocking TCP connection.
/// To implement this trait
/// specify ´SendType´ and ´ReceiveType´ to get ´run´.
pub trait TcpTransceive {
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

    pub fn run(self) -> ! {
        let mut buffer = String::new();
        let mut depth = 0;
        for c in self.stream.chars(){
            let c = c.unwrap();
            let depth_prev = depth;
            buffer.push(c);
            if c == '{' { depth += 1;} 
            if c == '}' { depth -= 1;}
            if (depth == 0) && (depth_prev == 1) {
                //println!("Common sender got {}",buffer.clone());
                let received: R = serde_json::from_str(buffer.as_str()).expect("Unable to deserialize");
                self.tx.send(received).expect("Unable to send received object");
                buffer.clear();
            }
        }
        /*
        loop {
            let mut recieved: [u8; 10000];
            let mut recieved = String::new();
            self.stream.read_to_string(&mut recieved).expect("Unable to read from Tcpstream");
            println!("Common sender got {}",recieved.clone());
            let recieved: R = serde_json::from_str(&recieved).expect("Unable to deserialize");
            self.tx.send(recieved).expect("Unable to send received object");
        }*/
        panic!("No more TCP");
    }
}


struct TcpSender<T> where T: Serialize {
    stream: TcpStream,
    rx: Receiver<T>,
}

impl<T> TcpSender<T> where T: Serialize {
     pub fn new(stream: TcpStream, rx: Receiver<T>) -> Self {
         //stream.set_nodelay(true);
         TcpSender{stream, rx}
     }

     pub fn run(mut self) -> ! {
         loop {
             let to_send = self.rx.recv().expect("Unable to receive something to send");
             let to_send = serde_json::to_string(&to_send).expect("Unable to serialize");
             self.stream.write(to_send.as_bytes()).expect("Unable to send object");
             //println!("Common sender sent {}",to_send);
         }
     }
}
