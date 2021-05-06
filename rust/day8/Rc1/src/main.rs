use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {
            value: val,
            next: Option::None,
        }
    }

    pub fn push(&mut self, val: T) ->() {

        let t = Node {
            value: val,
            next: Option::None,
        };

        self.next = Some(Box::new(t));
    }
}


// impl<T> Deref for Node<T> {
//     type Target = Option<Box<Node<T>>>;
//     fn deref(&self) -> &Self::Target {
//         & self.next
//     }
// }

// impl<T> DerefMut for Node<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.next
//     }
// }

fn main() {
    let mut rc1 = Node::new(11i32);
    rc1.push(5);

    println!("{:?}", rc1);
    match rc1.next {
        Some(x) => println!("{:?}", x),
        None => (),
    }
    let t: Option<String> = Option::Some("fsd".to_string());
    match t {
        Some(x) => println!("{:?}", x),
        _ => (),
    }
}
