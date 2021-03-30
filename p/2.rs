use std::option::ok_or;

fn main() {

    let i = Some(1);
    i.as_ref().map(|x| println!("{}", x));
    i.as_ref().and_then(|x| Some(x*2));
}