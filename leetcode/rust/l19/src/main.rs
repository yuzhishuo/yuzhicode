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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (&head, &head);
        let mut root = ListNode::new(0);
        let mut curr = &mut root;
        let mut count = 0;
        while count < n {
            count += 1;
            fast = &fast.as_ref().unwrap().next;
        }

        while fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            let val = slow.as_ref().unwrap().val;
            slow = &slow.as_ref().unwrap().next;
            curr.next = Some(Box::new(ListNode::new(val)));
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = slow.as_ref().unwrap().next.clone();
        root.next.clone()
    }
}

fn main() {
    println!("no test!");
}
