use std::thread;
use std::sync::{Arc, Mutex};


fn main() {

    let num = Arc::new(Mutex::new(0));

    let mut threads = vec![];

    for elem in 0..10 {
        let num = Arc::clone(&num);
        let thread_item = thread::spawn(move || {
           let mut num_inside = num.lock().unwrap();
           *num_inside += elem;
           println!("thread nums {} num: {}", elem, *num_inside)
        });
        threads.push(thread_item);
    }

    for elem in threads {
        elem.join();
    }

    println!("Main Thread, num: {}", *num.lock().unwrap());
}
