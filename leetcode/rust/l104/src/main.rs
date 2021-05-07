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

struct Solution1;
struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution1 {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut root = root;
        if let Some(ref cur) = root {
            let mut t = cur.borrow_mut();
            return std::cmp::max(
                Self::max_depth(t.left.take()),
                Self::max_depth(t.right.take()),
            ) + 1;
        }
        0
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut three_node_deque = VecDeque::<Rc<RefCell<TreeNode>>>::new();

        let mut ret = 1;

        three_node_deque.push_back(root.unwrap());

        while !three_node_deque.is_empty() {
            ret += 1;
            let mut new_deque = VecDeque::<Rc<RefCell<TreeNode>>>::new();

            for itr in three_node_deque.iter() {
                let mut cur = itr.borrow_mut();

                if let Some(left_v) = cur.left.take() {
                    new_deque.push_back(left_v);
                }

                if let Some(right_v) = cur.right.take() {
                    new_deque.push_back(right_v);
                }
            }
            three_node_deque = new_deque;
        }
        ret
    }
}

fn main() {
    println!("Hello, world!");
}
