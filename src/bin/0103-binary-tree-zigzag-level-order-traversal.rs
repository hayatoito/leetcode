// @leetup=info id=103 lang=rust slug=binary-tree-zigzag-level-order-traversal


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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q = Vec::new();
        if let Some(n) = root {
            q.push(n);
        }

        let mut res = Vec::<Vec<i32>>::new();

        while !q.is_empty() {
            if res.len() % 2 == 1 {
                res.push(q.iter().rev().map(|n| n.borrow().val).collect());
            } else {
                res.push(q.iter().map(|n| n.borrow().val).collect());
            }
            let mut nq = Vec::new();
            for n in q {
                if let Some(m) = &n.borrow().left {
                    nq.push(m.clone());
                }
                if let Some(m) = &n.borrow().right {
                    nq.push(m.clone());
                }
            }
            q = nq;
        }

        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::zigzag_level_order;
}
