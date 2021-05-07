use std::cell::RefCell;
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

struct BSTIterator {
    inner: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIterator { inner: vec![root] }
    }

    fn next(&mut self) -> i32 {
        let mut node = self.inner.pop().unwrap();
        while node.is_some() || !self.inner.is_empty() {
            while let Some(cur) = node {
                let left = cur.borrow_mut().left.take();
                self.inner.push(Some(cur));
                node = left;
            }
            if let Some(top) = self.inner.pop() {
                if let Some(cur) = top {
                    node = cur.borrow_mut().right.take();
                    if node.is_some() {
                        self.inner.push(node);
                    }
                    return cur.borrow().val;
                }
            }
        }
        return -1;
    }

    fn has_next(&self) -> bool {
        !self.inner.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

fn main() {
    println!("Hello, world!");
}
