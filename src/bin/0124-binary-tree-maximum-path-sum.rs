// @leetup=info id=124 lang=rust slug=binary-tree-maximum-path-sum


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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i32::MIN;
        fn max_path(node: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            if let Some(node) = node {
                let left = max_path(node.borrow().left.clone(), res);
                let right = max_path(node.borrow().right.clone(), res);
                *res = (*res).max(node.borrow().val + left + right);
                0.max(node.borrow().val + left.max(right))
            } else {
                0
            }
        }

        max_path(root, &mut res);
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::max_path_sum;
}
