
#[derive(Debug)]
struct Person<'a, T> {

    age: &'a T
}

fn main () {

    let age = 32i32;

    let tian_yi = Person {age : &age};


    print!("{:?}", TianYi);
}