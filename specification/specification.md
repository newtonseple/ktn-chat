# Specification

The chat program will be written in Rust. The information flow is mainly handled by channels between threads, and as such the arrows in the sequence diagrams represent dataflow, rather than function calls. 

## Common
As both the client and the server needs an asynchronous TCP-socket and Request and Response types these have been moved ou to a common library. The library contains the `Request` type, along with implementation of the `FromStr` trait, the `Response` type which implements the `Display` trait, so that it can be rintet, and the `TcpTranceive` trait which can be implemented for structs, so that they can act as a asynchronous TCP-socket. To be able to act as an asynchronous socket the `TcpTranceive` trait also uses two structs, that each wrap around the same `TcpStream`, where one of the structs are used for writing, and one for reading. The `run` function provided by the trait creates these structs in different threads, and give them the means to comunicate with the rest of the program.

```Rust
pub enum Request {
    login { content: Option<String> },
    logout { content: Option<String> },
    msg { content: Option<String> },
    names { content: Option<String> },
    help { content: Option<String> },
}
```
```Rust
pub enum Response {
    error {
        timestamp: String,
        sender: String,
        content: String,
    },
    info {
        timestamp: String,
        sender: String,
        content: String,
    },
    message {
        timestamp: String,
        sender: String,
        content: String,
    },
    history {
        timestamp: String,
        sender: String,
        content: Vec<Response>,
    },
}
}
```
```Rust
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
```

## Server
The server is what contains most of (if not all) the logic. On startup it will create a `ChatServer` that creates a `ClientManager` which will listen for, and accept incoming connection requests. For every established connection a `ServerTcpTranceiver`, a unit struct that implements the `TcpTranceive` trait, will be made, and it will run in a separate thread. The `Clienthandler` will recieve requests from a `TcpStream`, parse them, and hand them to the `ChatServer` via a channel. Likwise the `ChatServer` will hand responses through a channel to the `ClientHandler`. When the `ChatServer` recieves a request it will perform the necessary actions. A request will be on the form

The only requests that is valid for a user that is not yet logged in is `login` and `help`. All other responses will be answered with an error response.

## Client
The client will get an IP address and port of a listening server during startup. It then connects to the server, and pass messages between the user and the server. When the user writes a command on the form `<command> [data]` it will be parsed by the client, and sent to the server. Likewise when the client recieves a response form the server it will parse this response and show the relevant information to the user. To do this the client implements the `TcpTranceive` trait, and have to structs that handle reading to, and writing from the console. These struct live in their own threads so that the I/O is asyncronous.