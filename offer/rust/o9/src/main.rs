struct CQueue {
    stack: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    #[allow(dead_code)]
    fn new() -> Self {
        CQueue {
            stack: vec![vec![], vec![]],
        }
    }
    fn append_tail(&mut self, value: i32) {
        self.stack[0].push(value);
    }
    fn delete_head(&mut self) -> i32 {
        if self.stack[1].len() == 0 && self.stack[0].len() == 0 {
            return -1;
        }
        if self.stack[1].len() != 0 {
            return self.stack[1].pop().unwrap();
        }

        self.stack[0].reverse();
        self.stack[1] = self.stack[0].clone();
        self.stack[0].clear();
        self.stack[1].pop().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
