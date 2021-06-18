use List::*;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    // 注意 self 的修饰
    fn prepend(self, ele: i32) -> List {
        Cons(ele, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            Nil => 0,
            Cons(_, ref tail) => tail.len() + 1,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut head = List::new();

    head = head.prepend(123);

    head = head.prepend(44);

    println!("{}", head.len());
    println!("{}", head.stringify());
}
