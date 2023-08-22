// @leetup=info id=973 lang=rust slug=k-closest-points-to-origin

// @leetup=code
use std::collections::*;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut q = BinaryHeap::new();
        for p in points {
            q.push((p[0] * p[0] + p[1] * p[1], p));
            if q.len() > k as usize {
                q.pop();
            }
        }
        q.into_iter().map(|(_, p)| p).collect()
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::k_closest;
}
