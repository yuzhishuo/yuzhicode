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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::_is_symmetric_helper(root.clone(), root.clone());
    }

    fn _is_symmetric_helper(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if !p.is_some() || !q.is_some() {
            return false;
        }

        let (p, q) = (p.as_ref().unwrap().borrow(), q.as_ref().unwrap().borrow());

        p.val == q.val
            && Self::_is_symmetric_helper(p.left.clone(), q.right.clone())
            && Self::_is_symmetric_helper(p.right.clone(), q.left.clone())
    }
}

fn main() {
    println!("Hello, world!");
}
