// @leetup=info id=189 lang=rust slug=rotate-array

// @leetup=code

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::rotate;
}
