use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
struct List {
    val: i32,
    next: Option<Rc<Cell<List>>>,
}

fn main() {
    let t1 = Cell::new(1);
    {
        t1.set(12);

        print!("{}", t1.get());
    }

    let nodes = List {
        val: 1,
        next: Some(Rc::new(Cell::new(List {
            val: 2,
            next: Some(Rc::new(Cell::new(List { val: 3, next: None }))),
        }))),
    };

    if let Some(mut t) = nodes.next.clone() {
        t.borrow_mut().set(List { val: 2, next: None });
    }
}
