// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #![allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    #![allow(dead_code)]
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut head = head;
        let mut v = vec![];
        while head.is_some() {
            v.push(head.as_ref().unwrap().val);
            head = head.unwrap().next.take();
        }
        v.reverse();
        v
    }
}

fn main() {}
