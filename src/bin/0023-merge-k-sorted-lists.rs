// @leetup=info id=23 lang=rust slug=merge-k-sorted-lists


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// @leetup=code
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut head = &mut dummy_head;

        let mut heap = BinaryHeap::new();
        for n in lists {
            if let Some(n) = n {
                heap.push(Reverse(n));
            }
        }

        while let Some(Reverse(mut node)) = heap.pop() {
            if let Some(next_node) = node.next.take() {
                heap.push(Reverse(next_node));
            }
            head.next = Some(node);
            head = head.next.as_mut().unwrap();
        }

        return dummy_head.next;
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::merge_k_lists;
}
