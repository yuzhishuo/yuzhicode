use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(move || {
        for i in 1..=100 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(i as u64));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
