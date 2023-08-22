// @leetup=info id=230 lang=rust slug=kth-smallest-element-in-a-bst


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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, inorder: &mut Vec<i32>) {
            if let Some(root) = root {
                dfs(root.borrow().left.clone(), inorder);
                inorder.push(root.borrow().val);
                dfs(root.borrow().right.clone(), inorder);
            }
        }
        let mut inorder = Vec::new();
        dfs(root, &mut inorder);
        inorder[(k - 1) as usize]
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::kth_smallest;
}
