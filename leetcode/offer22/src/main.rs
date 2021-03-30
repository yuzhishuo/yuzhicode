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
  pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut new_head = Some(Box::new(ListNode { val: 0, next: head }));

    let mut head = &mut new_head;
    let mut i = 0;
    while let Some(node) = head {
      i += 1;
      head = &mut node.next;
    }
    let i = i - k;
    for _ in 0..i {
      new_head = new_head.as_mut().unwrap().next.take();
    }
    new_head.unwrap().next
  }
}

fn main() {
  println!("Hello, world!");
}
