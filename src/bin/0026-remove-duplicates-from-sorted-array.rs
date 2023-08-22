// @leetup=info id=26 lang=rust slug=remove-duplicates-from-sorted-array

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut left = 1;
        for right in 1..nums.len() {
            if nums[left - 1] < nums[right] {
                nums[left] = nums[right];
                left += 1;
            }
        }
        left as i32
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::remove_duplicates;
}
