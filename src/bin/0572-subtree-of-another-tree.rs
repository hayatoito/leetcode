// @leetup=info id=572 lang=rust slug=subtree-of-another-tree


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

#[allow(unused_imports)]
use std::collections::*;

use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {

        fn is_identical(a: Node, b: Node) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    a.borrow().val == b.borrow().val &&
                    is_identical(a.borrow().left.clone(), b.borrow().left.clone()) &&
                    is_identical(a.borrow().right.clone(), b.borrow().right.clone())
                }
                _ => false
            }
        }

        fn is_subtree(root: Node, subtree: Node) -> bool {
            match (root, subtree) {
                (_, None) => true,
                (None, _) => false,
                (Some(root), Some(subtree)) => {
                    is_identical(Some(root.clone()), Some(subtree.clone())) ||
                    is_subtree(root.borrow().left.clone(), Some(subtree.clone())) ||
                    is_subtree(root.borrow().right.clone(), Some(subtree))
                }
            }
        }

        is_subtree(root, sub_root)
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::is_subtree;
}
