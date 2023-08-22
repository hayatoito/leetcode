// @leetup=info id=234 lang=rust slug=palindrome-linked-list

// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

// @leetup=code

// time: O(n), space: O(1)

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let (mut a, b) = Solution::split_list(head);
        let mut b = Solution::reverse(b);
        while let (Some(an), Some(bn)) = (a, b) {
            if an.val != bn.val {
                return false;
            }
            // Unstable in leetcode.
            // (a, b) = (an.next, bn.next);
            a = an.next;
            b = bn.next;
        }
        true
    }

    fn list_len(mut head: &Option<Box<ListNode>>) -> usize {
        let mut cnt = 0;
        while let Some(node) = head.as_ref() {
            head = &node.next;
            cnt += 1;
        }
        cnt
    }

    fn split_list(
        mut head: Option<Box<ListNode>>,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let cnt = Solution::list_len(&head);
        let mut h = &mut head;
        for _ in 0..cnt / 2 {
            h = &mut h.as_mut().unwrap().next;
        }
        let b = h.take();
        (head, b)
    }

    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }
        prev
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::is_palindrome;
}
