// @leetup=info id=128 lang=rust slug=longest-consecutive-sequence

// @leetup=code
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let nums = nums.into_iter().collect::<HashSet<_>>();
        for n in &nums {
            let mut i = *n;
            if !nums.contains(&(i - 1)) {
                while nums.contains(&(i + 1)) {
                    i += 1;
                }
            }
            res = res.max(i - n + 1);
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::longest_consecutive;
}
