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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut first = Some(Box::new(ListNode::new(0)));
        let mut point = &mut first;
        let mut v = vec![];
        let mut head = head;
        loop {
            for _ in 0..k {
                if head.is_none() {
                    for node in v {
                        if let Some(p) = point {
                            p.next = Some(node);
                            point = &mut p.next;
                        }
                    }
                    return first.unwrap().next;
                }
                if let Some(mut p) = head {
                    head = p.next.take();
                    v.push(p);
                }
            }

            while !v.is_empty() {
                if let Some(p) = point {
                    p.next = v.pop();
                    point = &mut p.next;
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
