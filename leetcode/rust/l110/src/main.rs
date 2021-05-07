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
    pub fn is_balance(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(root) == -1
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut cur = root.as_ref().unwrap().borrow_mut();
        let (left, right) = (
            Self::helper(cur.left.take()),
            Self::helper(cur.right.take()),
        );

        if left == -1 || right == -1 || (left - right).abs() == 1 {
            return -1;
        }
        1 + left.max(right)
    }
}

fn main() {
    println!("Hello, world!");
}
