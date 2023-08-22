// @leetup=info id=235 lang=rust slug=lowest-common-ancestor-of-a-binary-search-tree


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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn lowest(root: Rc<RefCell<TreeNode>>, p: i32, q: i32) -> Rc<RefCell<TreeNode>> {
            let root_val = root.borrow().val;
            if root_val == p || root_val == q {
                root
            } else if p < root_val && root_val < q {
                root
            } else if p < root_val {
                lowest(root.borrow().left.as_ref().unwrap().clone(), p, q)
            } else {
                lowest(root.borrow().right.as_ref().unwrap().clone(), p, q)
            }
        }

        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        Some(lowest(root.unwrap(), p.min(q), p.max(q)))
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::lowest_common_ancestor;
}
