use std::sync::Arc;
use std::thread;

struct Mduo {

    name: &'static mut String
}

fn main() {
    let numbers :Vec<_> = (0..10u32).collect();

    let shared_numbers = Arc::new(numbers);

    let mut m :  String = "fsdf".to_string();
    // let c = Mduo{ name: &m };

    for _ in 0..10 {
        
        let child_numbers = shared_numbers.clone();

        thread::spawn(move || {
            let mut local_numbers = & child_numbers[..];
            
            let mut t =  &local_numbers[9];
    
            t=&100;
            println!("{:?}", t);
            println!("{:?}", local_numbers);
        });
    }
}
