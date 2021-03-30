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
    //! 递归解法
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut path: Vec<i32> = Vec::new();
        let root = &root;
        fn first(
            res: &mut Vec<Vec<i32>>,
            path: &mut Vec<i32>,
            root: &Option<Rc<RefCell<TreeNode>>>,
            target: i32,
        ) {
            match root {
                None => (return),
                Some(ref x) => {
                    path.push(x.borrow().val);
                    let next_target = target - x.borrow().val;
                    if next_target == 0 && x.borrow().left.is_none() && x.borrow().right.is_none() {
                        res.push(path.clone());
                        return;
                    }

                    first(res, path, &x.borrow().left, next_target);
                    first(res, path, &x.borrow().right, next_target);
                    path.pop();
                }
            };
        }
        first(res.as_mut(), path.as_mut(), root, target);
        res
    }
}

impl Solution {
    fn path_sum1(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {
        let mut path: Vec<i32> = vec![];
        let mut inner: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut node = root;
        let mut res: Vec<Vec<i32>> = vec![];
        while node.is_some() || !inner.is_empty() {
            while let Some(cur) = node {
                path.push(cur.borrow().val);
                node = cur.borrow_mut().left.take();
                inner.push(cur);
            }
            node = inner.pop();
            if let Some(top) = node {
                if top.borrow().right.is_some() {
                    inner.push(top.clone());
                    node = top.borrow_mut().right.take();
                } else {
                    if path.iter().fold(0, |acc, x| acc + x) == target {
                        res.push(path.clone());
                    }
                    path.pop();
                    node = None;
                    while let Some(cur) = inner.pop() {
                        path.pop();
                        if cur.borrow().right.is_some() {
                            node = Some(cur);
                            break;
                        }
                    }
                }
            }
        }
        res
    }
}

fn main() {}
