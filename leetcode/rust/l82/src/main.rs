use std::collections::HashSet;

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = ListNode { val: 0, next: head };

        let mut cur = &mut list;

        while cur.next.is_some() {
            let next_mut: &mut ListNode = cur.next.as_mut().unwrap();
            let mut repeated = i32::MIN;

            if cur.val == next_mut.val {
                repeated = cur.val;
            }

            if repeated == i32::MIN {
                cur = cur.next.as_mut().unwrap();
                continue;
            }
            let mut node = None;
            // reborrow2
            let mut fast: &mut ListNode = next_mut;
            while fast.next.is_some() {
                if let Some(fnext) = fast.next.as_mut() {
                    if fnext.val != repeated {
                        // 已经遍历了当前子链的重复元素
                        break;
                    }
                }

                // 否则下一个引用也是重复元素，往前迭代
                fast = fast.next.as_mut().unwrap();
            }

            node = fast.next.take();
            // reborrow2 fast end
            // reborrow1 next_mut end
            cur.next = node;
        }

        list.next
    }
}

fn main() {}
