use std::cell::RefCell;

struct Node<'a>(&'a mut RefCell<i32>);

fn main() {
    let mut n0 = RefCell::new(123);
    {
        let n1 = Node(&mut n0);

        *n1.0.get_mut() = 1;
        println!("{}", n1.0.borrow());
    }
    println!("{}", n0.borrow());
}
