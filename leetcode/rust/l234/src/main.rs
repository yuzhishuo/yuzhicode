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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }
        let (mut prev, mut next) = Self::split_list(head);
        while let Some(p) = prev {
            if p.val != next.as_ref().unwrap().val {
                return false;
            }
            prev = p.next;
            next = next.unwrap().next;
        }
        true
    }

    fn split_list(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut fast: *const _ = &head;
        let mut slow = head;
        let mut prev = None;
        let mut odd = false;

        while let Some(n1) = unsafe { &*fast } {
            fast = &n1.next;
            if let Some(n2) = unsafe { &*fast } {
                fast = &n2.next;
                let slow_next = slow.as_mut().unwrap().next.take();
                slow.as_mut().unwrap().next = prev;
                prev = slow;
                slow = slow_next;
            } else {
                odd = true;
            }
        }

        if odd {
            (prev, slow.unwrap().next)
        } else {
            (prev, slow)
        }
    }
}

fn main() {
    let mut node = ListNode::new(1);

    let mut node1 = Some(Box::new(ListNode::new(2)));
    let mut node2 = Some(Box::new(ListNode::new(2)));
    let mut node3 = Some(Box::new(ListNode::new(2)));
    let node4 = Some(Box::new(ListNode::new(1)));
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    node1.as_mut().unwrap().next = node2;
    node.next = node1;
    assert_eq!(Solution::is_palindrome(Some(Box::new(node))), true);
}
