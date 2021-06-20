// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    #![allow(dead_code)]
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l3 = ListNode::new(0);
        let mut ptr3 = &mut l3;
        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val < n2.val {
                ptr3.next = l1;
                ptr3 = ptr3.next.as_mut().unwrap();
                l1 = ptr3.next.take();
            } else {
                ptr3.next = l2;
                ptr3 = ptr3.next.as_mut().unwrap();
                l2 = ptr3.next.take();
            }
        }
        ptr3.next = if l1.is_some() { l1 } else { l2 };
        l3.next
    }
}

fn main() {
    println!("no test!");
}
