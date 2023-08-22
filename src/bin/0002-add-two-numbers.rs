// @leetup=info id=2 lang=rust slug=add-two-numbers


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

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut head = &mut dummy_head;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry == 1 {
            let mut digit = 0;
            if let Some(d) = l1 {
                digit += d.val;
                l1 = d.next;
            }
            if let Some(d) = l2 {
                digit += d.val;
                l2 = d.next;
            }
            digit += carry;
            carry = digit / 10;
            digit = digit % 10;

            let node = Some(Box::new(ListNode::new(digit)));
            head.next = node;
            head = head.next.as_mut().unwrap();
        }
        dummy_head.next
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::add_two_numbers;
}
