// @leetup=info id=347 lang=rust slug=top-k-frequent-elements

// @leetup=code
use std::collections::*;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();
        for n in nums {
            *count.entry(n).or_insert(0) += 1;
        }
        let mut q = BinaryHeap::new();
        for (n, count) in count {
            q.push((-count, n));
            if q.len() > k as usize {
                q.pop();
            }
        }
        q.into_iter().map(|(_, n)| n).collect()
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::top_k_frequent;
}
