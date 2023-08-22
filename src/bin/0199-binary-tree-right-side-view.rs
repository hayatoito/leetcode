// @leetup=info id=199 lang=rust slug=binary-tree-right-side-view


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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<i32>) {
            if let Some(root) = root {
                if res.len() == level {
                    res.push(root.borrow().val);
                }
                dfs(root.borrow().right.clone(), level + 1, res);
                dfs(root.borrow().left.clone(), level + 1, res);
            }
        }

        let mut res = Vec::new();
        dfs(root, 0, &mut res);
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::right_side_view;
}
