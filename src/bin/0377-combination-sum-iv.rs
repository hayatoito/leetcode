// @leetup=info id=377 lang=rust slug=combination-sum-iv

// @leetup=code

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let nums = nums.into_iter().map(|n| n as usize).collect::<Vec<_>>();

        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for i in 0..target {
            for n in &nums {
                if i + n <= target {
                    dp[i + n] += dp[i];
                }
            }
        }
        dp[target]
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::combination_sum4;
}
