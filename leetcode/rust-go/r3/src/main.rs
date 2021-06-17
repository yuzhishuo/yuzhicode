use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug)]
struct Vbox<T:  Display + Debug>(T);

impl<T: Display + Debug> Vbox<T> {
    fn new(val: T) -> Vbox<T> {
        Vbox(val)
    }
}

impl<T: Display + Debug> Deref for Vbox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Display + Debug> Drop for Vbox<T> {
    fn drop(&mut self) {
        println!("drpo, {:?}", self);
    }
}

fn derefcovert(i: &i32) {
    println!("Hello, {}!", i);
}

fn main() {
    let t = Vbox::new(2);
    let t1 = Vbox::new(1);
    derefcovert(&t1);
    println!("{}", *t);
}
