// @leetup=info id=279 lang=rust slug=perfect-squares

// @leetup=code

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![n; n + 1];
        dp[0] = 0;
        for i in 1..n + 1 {
            for j in 1..n + 1 {
                if j * j <= i {
                    dp[i] = dp[i].min(dp[i-j*j] + 1);
                } else {
                    break
                }
            }
        }
        dp[n] as i32
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::num_squares;
}
