// @leetup=info id=1143 lang=rust slug=longest-common-subsequence

// @leetup=code
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for (i, a) in text1.chars().enumerate() {
            for (j, b) in text2.chars().enumerate() {
                dp[i + 1][j + 1] = if a == b {
                    dp[i][j] + 1
                } else {
                    dp[i + 1][j].max(dp[i][j + 1])
                }
            }
        }
        dp[text1.len()][text2.len()]
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::longest_common_subsequence;
}
