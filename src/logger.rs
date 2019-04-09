use std::fs::File;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::io::Write;

pub fn start(logger_name: &str) -> Sender<String> {
    let mut file = File::create(logger_name).unwrap();
    let (send_chan, rcv_chan) = mpsc::channel::<String>();
    thread::spawn(move || {
        loop {
            match rcv_chan.recv() {
                Ok(log_message) => file.write_all(log_message.as_bytes()).unwrap(),
                Err(_) => break,
            }
        }
    });
    send_chan
}
