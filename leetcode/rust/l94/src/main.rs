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
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let mut root = root;

        let mut res: Vec<i32> = vec![];
        let mut stack = vec![root.take().unwrap()];

        while !stack.is_empty() {
            let mut t = stack.last().unwrap().clone();

            while t.borrow().left.is_some() {
                stack.push(t.borrow_mut().left.take().unwrap());
                t = stack.last().unwrap().clone();
            }

            let t = stack.pop().unwrap();
            res.push(t.borrow().val);
            if t.borrow().right.is_some() {
                stack.push(t.borrow_mut().right.take().unwrap());
            }
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
