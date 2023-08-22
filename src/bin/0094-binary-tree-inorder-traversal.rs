// @leetup=info id=94 lang=rust slug=binary-tree-inorder-traversal


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

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn visit(n: Option<Rc<RefCell<TreeNode>>>, xs: &mut Vec<i32>) {
            if let Some(n) = n {
                visit(n.borrow().left.clone(), xs);
                xs.push(n.borrow().val);
                visit(n.borrow().right.clone(), xs);
            }
        }

        let mut res = Vec::new();
        visit(root, &mut res);
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::inorder_traversal;
}
