// @leetup=info id=104 lang=rust slug=maximum-depth-of-binary-tree


// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root) = root {
                1 + dfs(root.borrow().left.clone()).max(dfs(root.borrow().right.clone()))
            } else {
                0
            }
        }

        dfs(root)
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::max_depth;
}
