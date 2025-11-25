use std::sync::mpsc::{self, RecvError};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), RecvError> {
    let (tx, rx) = mpsc::channel();
    let _handle = thread::spawn(move || {
        let vals = [
            String::from("Hello"),
            String::from("Hello"),
            String::from("Hello"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            std::thread::sleep(Duration::from_secs(1));
        }
    });
    for recived in rx {
        println!("recived : {:?}", recived);
    }
    Ok(())
}
