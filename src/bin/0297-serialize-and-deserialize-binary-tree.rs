// @leetup=info id=297 lang=rust slug=serialize-and-deserialize-binary-tree


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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(n: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<String>) {
            if let Some(n) = n {
                vals.push(n.borrow().val.to_string());
                dfs(n.borrow().left.clone(), vals);
                dfs(n.borrow().right.clone(), vals);
            } else {
                vals.push("null".to_string());

            }
        }

        let mut vals = Vec::new();
        dfs(root, &mut vals);
        vals.join(" ")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn doit<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<Rc<RefCell<TreeNode>>> {
            let val = iter.next().unwrap();
            if val == "null" {
                None
            } else {
                let mut n = TreeNode::new(val.parse().unwrap());
                n.left = doit(iter);
                n.right = doit(iter);
                Some(Rc::new(RefCell::new(n)))
            }

        }

        doit(&mut data.split(" "))
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
// @leetup=code

fn main() {
    let _ = Codec::new;
    let _ = Codec::serialize;
    let _ = Codec::deserialize;
}
