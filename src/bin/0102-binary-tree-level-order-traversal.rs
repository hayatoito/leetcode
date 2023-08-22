// @leetup=info id=102 lang=rust slug=binary-tree-level-order-traversal


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
use std::collections::*;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root == None {
            return vec![];
        }
        let mut cur_q = VecDeque::new();
        cur_q.push_back(root.unwrap());
        let mut next_q = VecDeque::new();
        let mut result: Vec<Vec<i32>> = Vec::new();

        while !cur_q.is_empty() {
            result.push(cur_q.iter().map(|n| n.borrow().val).collect());
            while let Some(n) = cur_q.pop_front() {
                if let Some(n) = &n.borrow().left {
                    next_q.push_back(n.clone());
                }
                if let Some(n) = &n.borrow().right {
                    next_q.push_back(n.clone());
                }
            }
            std::mem::swap(&mut cur_q, &mut next_q);
        }
        result
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::level_order;
}
