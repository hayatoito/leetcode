// @leetup=info id=91 lang=rust slug=decode-ways

// @leetup=code
use std::collections::*;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let valid = (1..27).map(|i| i.to_string()).collect::<HashSet<_>>();
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        for i in 1..s.len() + 1 {
            for j in 0..i {
                if valid.contains(&s[j..i]) {
                    dp[i] += dp[j];
                }
            }
        }
        dp[s.len()]
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::num_decodings;
}
