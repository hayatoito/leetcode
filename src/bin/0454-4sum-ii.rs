// @leetup=info id=454 lang=rust slug=4sum-ii

// @leetup=code

use std::collections::*;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let cnt1 = count(&nums1, &nums2);
        let cnt2 = count(&nums3, &nums4);
        let mut res = 0;
        for (k1, v1) in cnt1 {
            if let Some(v2) = cnt2.get(&-k1) {
                res += v1 * v2;
            }
        }
        res as i32
    }
}

fn count(nums1: &[i32], nums2: &[i32]) -> HashMap<i32, usize> {
    let mut res = HashMap::new();
    for i in nums1 {
        for j in nums2 {
            *res.entry(i + j).or_insert(0) += 1;
        }
    }
    res
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::four_sum_count;
}
