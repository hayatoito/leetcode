// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
    #[allow(dead_code)]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {

    let _s = Solution;
}

#[allow(dead_code)]
struct Solution;


impl Solution {
    #[allow(dead_code)]
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut current = head.as_mut().unwrap();

        while let Some(next) = current.next.as_mut() {
            if current.val == next.val {
                current.next = next.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        head
    }
}
