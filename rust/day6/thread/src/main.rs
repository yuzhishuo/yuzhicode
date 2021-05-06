use std::thread;
use thread::sleep;
use thread::JoinHandle;


fn main() {

    let mut threads: Vec<JoinHandle<()>>  = vec![];
    for  i in 0..10 {
        let hander_thread: JoinHandle<()> =  thread::spawn(move || {
            sleep(std::time::Duration::from_micros(10));
            println!("thread Id :{}", i);
        });

        threads.push(hander_thread);
    }

    for itr in threads {
        itr.join().expect("throw error");
    }
    println!("Main Thread");
}
