use std::sync::mpsc::{self, RecvError};
use std::thread;

fn main() -> Result<(), RecvError> {
    let (tx, rx) = mpsc::channel();
    let _handle = thread::spawn(move || {
        let val = String::from("Hello");
        tx.send(val).unwrap();
    });
    let value = rx.recv()?;
    println!("{:?}", value);
    Ok(())
}
