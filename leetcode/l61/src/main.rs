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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut fast = head.as_ref();
        let mut n = 0;
        while let Some(x) = fast {
            fast = x.next.as_ref();
            n += 1;
        }

        let k = k % n;
        if k == 0 {
            return head;
        }
        let k = n - k;
        let mut i = 0;
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut ptr = dummy.as_mut();
        while i < k {
            ptr = ptr.unwrap().next.as_mut();
            i = i + 1;
        }

        let mut new_head = ptr.unwrap().next.take();
        let mut ptr = new_head.as_mut();

        while ptr.is_some() && ptr.as_ref().unwrap().next.is_some() {
            ptr = ptr.unwrap().next.as_mut();
        }

        ptr.unwrap().next = dummy.unwrap().next.take();

        new_head
    }
}

fn main() {
    println!("Hello, world!");
}
