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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        // let  root = root;
        let mut res = vec![];
        let mut deuqe = VecDeque::new();
        deuqe.push_back(root.unwrap());
        while !deuqe.is_empty() {
            let mut deuqe1 = VecDeque::new();
            while !deuqe.is_empty() {
                if let Some(cur) = deuqe.pop_front() {
                    if let Some(left) = cur.borrow_mut().left.take() {
                        deuqe1.push_back(left);
                    }
                    if let Some(right) = cur.borrow_mut().right.take() {
                        deuqe1.push_back(right);
                    }
                    if deuqe.is_empty() {
                        res.push(cur.borrow().val);
                    }
                }
            }
            deuqe = deuqe1;
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
