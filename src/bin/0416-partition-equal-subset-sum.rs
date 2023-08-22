// @leetup=info id=416 lang=rust slug=partition-equal-subset-sum

// @leetup=code
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let len = (nums.iter().cloned().sum::<i32>() + 1) as usize;
        let mut dp = vec![false; len];
        dp[0] = true;

        for n in nums {
            let n = n as usize;
            let mut next_dp = vec![false; len];
            // for m in dp.into_iter().enumerate().filter(|(_, p)| *p).map(|(i, _)| i) {
            for m in dp
                .into_iter()
                .enumerate()
                .flat_map(|(m, p)| if p { Some(m) } else { None })
            {
                next_dp[(m + n)] = true;

                // Unstable:
                // next_dp[m.abs_diff(n)] = true;

                if m > n {
                    next_dp[m - n] = true;
                } else {
                    next_dp[n - m] = true;
                }
            }
            dp = next_dp;
        }

        dp[0]
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::can_partition;
}
