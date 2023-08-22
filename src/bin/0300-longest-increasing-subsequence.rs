// @leetup=info id=300 lang=rust slug=longest-increasing-subsequence

// @leetup=code
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp.into_iter().max().unwrap() as i32
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::length_of_lis;
}
