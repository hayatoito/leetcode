// @leetup=info id=28 lang=rust slug=find-the-index-of-the-first-occurrence-in-a-string

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(i) => i as i32,
            None => -1,
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::str_str;
}
