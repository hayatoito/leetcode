// @leetup=info id=148 lang=rust slug=sort-list

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

// MergeSort

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::merge_sort(head)
    }

    fn merge_sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match Solution::divide(head) {
            (Some(a), Some(b)) => {
                let a = Solution::merge_sort(Some(a));
                let b = Solution::merge_sort(Some(b));
                Solution::merge(a, b)
            }
            (Some(a), None) => Some(a),
            (None, Some(_)) => unreachable!(),
            (None, None) => None,
        }
    }

    fn divide(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut n = head.as_ref();
        let mut len = 0;
        while let Some(m) = n {
            n = m.next.as_ref();
            len += 1;
        }

        let mut n = head.as_mut();
        for _ in 0..(len - 1) / 2 {
            n = n.unwrap().next.as_mut();
        }

        let latter = n.and_then(|n| n.next.take());
        (head, latter)
    }

    fn merge(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(-1);
        let mut head = &mut dummy_head;

        while let (Some(x), Some(y)) = (a.as_mut(), b.as_mut()) {
            if x.val < y.val {
                let next = x.next.take();
                head.next = a.take();
                head = head.next.as_mut().unwrap();
                a = next;
            } else {
                let next = y.next.take();
                head.next = b.take();
                head = head.next.as_mut().unwrap();
                b = next;
            }
        }
        if let Some(a) = a {
            head.next = Some(a);
        } else if let Some(b) = b {
            head.next = Some(b);
        }

        dummy_head.next
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::sort_list;
}
