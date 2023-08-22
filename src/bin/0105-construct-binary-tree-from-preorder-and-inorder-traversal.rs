// @leetup=info id=105 lang=rust slug=construct-binary-tree-from-preorder-and-inorder-traversal


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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if inorder.is_empty() {
                return None;
            }
            let root = preorder[0];
            let root_index_in_inorder = inorder.iter().position(|&i| i == root).unwrap();

            let mut tree_node = TreeNode::new(root);
            tree_node.left = build(
                &preorder[1..root_index_in_inorder + 1],
                &inorder[..root_index_in_inorder],
            );
            tree_node.right = build(
                &preorder[root_index_in_inorder + 1..],
                &inorder[root_index_in_inorder + 1..],
            );
            Some(Rc::new(RefCell::new(tree_node)))
        }

        build(&preorder, &inorder)
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::build_tree;
}
