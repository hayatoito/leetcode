// @leetup=info id=101 lang=rust slug=symmetric-tree


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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn doit(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (left, right) {
                (Some(left), Some(right)) => {
                    left.borrow().val == right.borrow().val
                        && doit(left.borrow().left.clone(), right.borrow().right.clone())
                        && doit(left.borrow().right.clone(), right.borrow().left.clone())
                }
                (None, None) => true,
                _ => false,
            }
        }

        match root {
            Some(n) => doit(n.borrow().left.clone(), n.borrow().right.clone()),
            None => unreachable!(),
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::is_symmetric;
}
