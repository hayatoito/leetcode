// @leetup=info id=328 lang=rust slug=odd-even-linked-list

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

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut dummy_odd = Box::new(ListNode::new(0));
        let mut dummy_even = Box::new(ListNode::new(0));

        let mut odd = &mut dummy_odd;
        let mut even = &mut dummy_even;
        let mut is_odd = true;

        while let Some(n) = head {
            if is_odd {
                odd.next = Some(n);
                odd = odd.next.as_mut().unwrap();
                head = odd.next.take();
            } else {
                even.next = Some(n);
                even = even.next.as_mut().unwrap();
                head = even.next.take();
            }
            is_odd = !is_odd;
        }
        odd.next = dummy_even.next;
        dummy_odd.next
    }
}

// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::odd_even_list;
}
