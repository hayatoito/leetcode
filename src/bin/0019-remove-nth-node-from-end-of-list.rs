// @leetup=info id=19 lang=rust slug=remove-nth-node-from-end-of-list


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::remove_nth_from_end;
}
