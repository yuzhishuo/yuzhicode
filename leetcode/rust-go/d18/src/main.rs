
#[allow(dead_code)]
fn foo(add: u8) -> impl Fn(u8) -> u8 {
    move |origin: u8| origin + add
}

trait Trait {
    type Output;
    fn cc() -> Self::Output;
}

impl Trait for i32 {
    type Output = i32;
    fn cc() -> i32
        where Self::Output: std::fmt::Debug,
        Self: Sized
    {
        1
    }
}

trait Trait1 {}

impl Trait1 for i32 {}

#[allow(dead_code)]
fn function2() -> Box<impl Trait1> {
    Box::new(1)
}

fn main() {}
