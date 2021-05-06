use std::rc::Rc;
use std::rc::Weak;

fn main() {

    let mut five: Rc<i32> = Rc::new(5i32);

    let mut f1 = five.clone();
    
    let f2 = five.clone();

    println!("{:?}", Rc::strong_count(&five));

    f1 = Rc::new(3);
    
    println!("{:?}", Rc::strong_count(&five));
    println!("{:?}", Rc::strong_count(&f1));
}