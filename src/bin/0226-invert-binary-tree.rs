// @leetup=info id=226 lang=rust slug=invert-binary-tree


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

// @leetup=code
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let root = root.borrow();
            let mut node = TreeNode::new(root.val);
            node.borrow_mut().left = Self::invert_tree(root.right.clone());
            node.borrow_mut().right = Self::invert_tree(root.left.clone());
            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::invert_tree;
}
