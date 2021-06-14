// Definition for singly-linked list.
use std::cmp::{Ord, Ordering, PartialEq};
use std::collections::BinaryHeap;
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

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 默认是最大堆，这里颠倒顺序，实现最小堆。
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut ans = Box::new(ListNode::new(0));
        let mut ptr = &mut ans;
        let mut heap = BinaryHeap::new();
        for node in lists {
            if let Some(n) = node {
                heap.push(n);
            }
        }
        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(next);
            }
            ptr.next = Some(node);
            ptr = ptr.next.as_mut().unwrap();
        }

        ans.next
    }
}

fn main() {
    println!("Hello, world!");
}
