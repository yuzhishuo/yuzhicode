mod data;
use test;


#[derive(PartialEq, Eq, Debug)]
struct TestStruct {
    a : i32,
}

fn main() {

    let t = vec![1,3,2];
    let a = TestStruct{ a: 1};
    let b = TestStruct{ a: 1};
    let mut t =  data::DataStruct::Stack::<&TestStruct>::new();

    t.push(&a);
    t.push(&b);

    let c = t.pop();

    match c {
        None => (),
        Some(mut x) => println!("{}", x.a)
    }
    println!("Hello, world!");
}
