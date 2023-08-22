// @leetup=info id=139 lang=rust slug=word-break

// @leetup=code
use std::collections::*;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        let word_dict: HashSet<String> = word_dict.into_iter().collect();
        for end in 1..s.len() + 1 {
            for start in 0..end {
                if dp[start] && word_dict.contains(&s[start..end]) {
                    dp[end] = true;
                    break;
                }
            }
        }
        dp[s.len()]
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::word_break;
}
