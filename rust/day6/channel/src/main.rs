use std::thread;

use std::sync::mpsc;
use std::time::Duration; 


const THREAD_NUMBER: usize  =20;   


fn thread_start (d: usize, s: mpsc::Sender<usize>) {

    std::thread::spawn(move || {
        print!(" sleep time : {}", d);
        thread::sleep(Duration::from_micros(d as u64));
        println!(" sender usize: {} ", d);
        s.send(d).unwrap();
    });

} 

fn main() {

    let (sender, receiver) = mpsc::channel();

    for elem in 0 .. THREAD_NUMBER {
        thread_start(elem, sender.clone());
    }

    for elem in receiver.iter().take(THREAD_NUMBER) {
        println!("{}", elem);
    }
    println!("Hello, world!");
}
