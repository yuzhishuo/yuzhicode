use std::cmp::{Eq, Ord, Ordering};
use std::collections::BinaryHeap;

struct Node<T: Ord + Copy + Clone> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy + Clone> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl<T: Ord + Copy + Clone> Eq for Node<T> {}

impl<T: Ord + Copy + Clone> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl<T: Ord + Copy + Clone> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

struct Link<T: Ord + Copy + Clone> {
    node: Option<Box<Node<T>>>,
    length: usize,
}

impl<T: Ord + Copy + Clone> Node<T> {
    pub fn new(val: T) -> Node<T> {
        Node {
            val: val,
            next: None,
        }
    }

    pub fn connect(&mut self, val: T) {
        self.next = Some(Box::new(Self::new(val)));
    }
}

impl<T: Ord + Copy + Clone> Link<T> {
    pub fn new() -> Link<T> {
        Link {
            node: None,
            length: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        let mut last = &mut self.node;

        while let Some(x) = last {
            last = &mut x.next;
        }

        *last = Some(Box::new(Node::new(val)));
        self.length += 1;
    }
}

fn main() {
    let mut l = Link::<i32>::new();

    l.push(1);
    l.push(2);
    l.push(3);
    l.push(4);
    l.push(5);

    let mut l1 = Link::<i32>::new();

    l1.push(9);
    l1.push(10);
    l1.push(11);
    l1.push(12);
    l1.push(13);

    let v = vec![l, l1];

    let mut hp = BinaryHeap::<Box<Node<i32>>>::new();

    for itr in v {
        let mut next = itr.node;

        while let Some(mut x) = next {
            next = x.next.take();

            hp.push(x);
        }
    }

    let mut head = Link::<i32>::new();

    while !hp.is_empty() {
        let t = hp.pop();

        println!("{}", t.as_ref().unwrap().val);
        head.push(t.unwrap().val);
    }
}
