use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(ref cur) = root {
            return Self::path_sum_start_with_root(root.clone(), sum)
                + Self::path_sum(cur.borrow().left.clone(), sum)
                + Self::path_sum(cur.borrow().right.clone(), sum);
        }
        0
    }

    fn path_sum_start_with_root(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.as_ref().unwrap().borrow_mut();
        let mut count = if root.val == sum { 1 } else { 0 };

        count += Self::path_sum_start_with_root(root.left.clone(), sum - root.val);
        count += Self::path_sum_start_with_root(root.right.clone(), sum - root.val);
        count
    }
}

fn main() {
    println!("Hello, world!");
}
