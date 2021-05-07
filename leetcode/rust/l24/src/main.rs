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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_node = Some(Box::new(ListNode::new(-1)));
        let mut last_node = &mut new_node;

        let mut head = head;
        while let Some(ref mut curr) = head {
            let mut two = curr.next.take();
            let mut one = head.take();

            if let Some(ref mut here) = one {
                here.next = None;
            }

            if let Some(ref mut here) = two {
                head = here.next.take();
                here.next = one;
                last_node.as_mut().unwrap().next = two;
                last_node = &mut last_node.as_mut().unwrap().next.as_mut().unwrap().next;
            } else {
                head = None;
                last_node.as_mut().unwrap().next = one;
                last_node = &mut last_node.as_mut().unwrap().next;
            }
        }

        new_node.unwrap().next
    }
}
fn main() {}
