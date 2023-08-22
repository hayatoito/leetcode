// @leetup=info id=98 lang=rust slug=validate-binary-search-tree


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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_valid(root: Option<Rc<RefCell<TreeNode>>>, low: i64, high: i64) -> bool {
            if let Some(root) = root {
                let val = root.borrow().val as i64;
                low < val
                    && val < high
                    && is_valid(root.borrow().left.clone(), low, val)
                    && is_valid(root.borrow().right.clone(), val, high)
            } else {
                true
            }
        }
        is_valid(root, i64::MIN, i64::MAX)
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::is_valid_bst;
}
