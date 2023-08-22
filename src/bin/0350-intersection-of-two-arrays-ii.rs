// @leetup=info id=350 lang=rust slug=intersection-of-two-arrays-ii

// @leetup=code

use std::collections::*;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let cnt1 = cnt(&nums1);
        let cnt2 = cnt(&nums2);
        let mut res = Vec::new();
        for (k, v1) in cnt1 {
            if let Some(v2) = cnt2.get(&k) {
                res.append(&mut vec![k; v1.min(*v2)]);
            }
        }
        res
    }
}

fn cnt(nums: &[i32]) -> HashMap<i32, usize> {
    nums.iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(*n).or_insert(0) += 1;
        acc
    })
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::intersect;
}
