use std::sync::mpsc::{channel, Receiver, Sender};
use std::io;
use std::io::BufRead;


struct ConsoleReader;

impl ConsoleReader {
    fn run(console_input_tx: Sender<String>) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("Unable to read from stdin");
            console_input_tx.send(line).expect("Unable to send read string");
        }
    }
}

struct ConsolePrinter;

impl ConsolePrinter {
    //fn run()
}