use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display, U: Debug>(t: T, u: U) {
    println!("{}, {:?}", t, u);
}
#[derive(Debug)]
struct S(i32);

fn main() {
    compare_prints(String::from("23"), S(1));
}
