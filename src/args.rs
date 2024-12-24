
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{io, thread, time};

use crate::error::Error;

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
//
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub path: String,
    pub name: String,
}
//
pub fn get_args() -> Result<Message, Error> {
    let message: Message;
    let stdin_channel = spawn_stdin_channel();
    thread::sleep(time::Duration::from_millis(100));
    match stdin_channel.try_recv() {
        Ok(input) => {
            println!("read from stdin: {input}");
            message = serde_json::from_str(&input)?;
            println!("io::stdin(): {:?}", message);
            return Ok(message);
        }
        Err(error) => {
            println!("error read from stdin!: {error}");
            return Err(Error::FromString(format!("error read from stdin!: {error}")));
        }
    }
}
