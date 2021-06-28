use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    #[allow(unused_variables)]
    let handle = thread::spawn(move || {
        let value = String::from("hi");
        tx.send(value).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
