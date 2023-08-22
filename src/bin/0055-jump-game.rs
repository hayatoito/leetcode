// @leetup=info id=55 lang=rust slug=jump-game

// @leetup=code

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let nums_len = nums.len();
        let mut right = nums[0] as usize;
        for (left, n) in nums.into_iter().enumerate() {
            if right >= nums_len - 1 {
                return true;
            }
            right = right.max(left + n as usize);
            if left == right {
                return false;
            }
        }
        unreachable!();
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::can_jump;
}
