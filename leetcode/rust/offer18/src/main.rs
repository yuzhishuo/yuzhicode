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
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { next: head, val: 0 }));
        let mut fast = head.as_mut();
        while fast.as_ref().unwrap().next.is_some() {
            if fast.as_ref().unwrap().next.as_ref().unwrap().val == val {
                fast.as_mut().unwrap().next = fast.as_mut().unwrap().next.take().unwrap().next;
            }

            fast = fast.unwrap().next.as_mut();
        }
        head.unwrap().next
    }
}

impl Solution {
    pub fn delete_node1(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // Passed 0ms 2.1mb
        if head.is_none() {
            return head;
        }
        if head.as_ref().unwrap().val == val {
            return head.unwrap().next;
        }
        let mut head = head;
        let mut node = head.as_mut();
        while let Some(prev) = node {
            match prev.next.as_mut() {
                None => break,
                Some(cur) => {
                    if cur.val == val {
                        prev.next = cur.next.take();
                        break;
                    }
                }
            }
            node = prev.next.as_mut();
        }
        head
    }
}

fn main() {
    println!("Hello, world!");
}
