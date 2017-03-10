struct ChatServer {}

impl ChatServer {
    pub fn run() {
        set up listener
        set up log file
        set up client-list (name, tx, rx)
        loop {
            accept connections -> make handler (give channels)
            listen to channels
                log
                broadcast & respond
        }
    }

    fn helpers??
}

struct ClientHandler {}

impl ClientHandler {
    pub fn handle_client {
        loop {
            listen to tcp, channel       
            send to concerned
        }
    }
}

--------------------------------------------------------------------

struct ChatClient {}

impl ChatClient {
    pub fn run(),
    fn parse_input(),
    fn handle_server_message(),
}