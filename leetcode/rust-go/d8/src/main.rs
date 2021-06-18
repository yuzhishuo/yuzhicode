use std::fmt;
use std::fmt::Debug;

#[allow(dead_code)]
fn printer<T: fmt::Display>(_t: T) {}

#[allow(dead_code)]
struct S<T: fmt::Display>(T);

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Reactangle {
    width : f32,
    height: f32
}

impl HasArea for Reactangle {
    fn area(&self) ->f64 {
        (self.width * self.height) as f64
    }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn main() {
    print_debug(&Reactangle{ width : 1.0f32, height :23.0f32 });
}
