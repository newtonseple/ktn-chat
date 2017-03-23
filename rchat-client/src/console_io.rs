use std::sync::mpsc::{Receiver, Sender};
use std::io;
use std::io::BufRead;


pub struct ConsoleReader;

impl ConsoleReader {
    pub fn run(console_input_tx: Sender<String>) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("Unable to read from stdin");
            console_input_tx.send(line).expect("Unable to send read string");
        }
    }
}

pub struct ConsolePrinter;

impl ConsolePrinter {
    pub fn run(console_output_rx: Receiver<String>) {
        loop {
            println!("{}", console_output_rx.recv().expect("Unable to receive anything to print"));
        }
    }
}