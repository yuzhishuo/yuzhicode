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
    pub fn kth_largest1(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        if k == 0 {
            return root.unwrap().borrow().val;
        }

        let mut v = vec![];
        Self::scan(root, &mut v, k as usize);
        v.sort_by(|a, b| b.cmp(a));
        v[(k - 1) as usize]
    }
    fn scan(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>, k: usize) {
        let root = root.unwrap();

        if RefCell::borrow(&root).right.is_some() {
            let right = root.borrow_mut().right.take();
            Self::scan(right, v, k as usize);
        }
        v.push(root.borrow().val);
        if RefCell::borrow(&root).left.is_some() {
            let right = root.borrow_mut().left.take();
            Self::scan(right, v, k as usize);
        }
    }
}


impl Solution {
    pub fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        // Passed 0ms 2.8mb
        fn mfs(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
            let node = node.as_ref().unwrap().borrow();
            if node.right.is_some() { mfs(node.right.clone(), values); }
            values.push(node.val);
            if node.left.is_some() { mfs(node.left.clone(), values); }
        }

        let mut values = vec![];
        mfs(root, &mut values);
        values[k as usize - 1]
    }
}

fn main() {
    println!("Hello, world!");
}
