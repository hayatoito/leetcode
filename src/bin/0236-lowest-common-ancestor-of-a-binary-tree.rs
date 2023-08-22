// @leetup=info id=236 lang=rust slug=lowest-common-ancestor-of-a-binary-tree


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
        let mut found = None;
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        // Returns true if subtree contains p or q.
        fn includes_p_or_q(
            root: Option<Rc<RefCell<TreeNode>>>,
            p_val: i32,
            q_val: i32,
            found: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if let Some(root) = root {
                let mut cnt = 0;
                let root_val = root.borrow().val;
                if root_val == p_val || root_val == q_val {
                    cnt += 1;
                }
                if includes_p_or_q(root.borrow().left.clone(), p_val, q_val, found) {
                    cnt += 1;
                }
                if includes_p_or_q(root.borrow().right.clone(), p_val, q_val, found) {
                    cnt += 1;
                }
                if cnt == 2 {
                    *found = Some(root.clone());
                }
                cnt > 0
            } else {
                false
            }
        }

        includes_p_or_q(root, p_val, q_val, &mut found);
        found
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::lowest_common_ancestor;
}
