// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    // error
    #[allow(dead_code)]
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None || head.as_ref().unwrap().next == None {
            return head;
        }
        let mut head = head;
        let np = Self::reverse_list(head.as_mut().unwrap().next.take());
        head.and_then(move |mut x| {
            x.next.as_mut().unwrap().next = Some(x.clone());
            x.next = None;
            np
        })
    }
    #[allow(dead_code)]
    pub fn reverse_list1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None || head.as_ref().unwrap().next == None {
            return head;
        }
        let mut lhs = head;
        let mut rhs = None;

        while let Some(mut node) = lhs {
            lhs = node.next.take();
            node.next = rhs;
            rhs = Some(node);
        }
        rhs
    }
}

fn main() {
    println!("Hello, world!");
}
