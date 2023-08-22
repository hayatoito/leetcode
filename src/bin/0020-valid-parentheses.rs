// @leetup=info id=20 lang=rust slug=valid-parentheses

// @leetup=code
use std::collections::*;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let mut pairs = HashMap::new();
        pairs.insert(')', '(');
        pairs.insert('}', '{');
        pairs.insert(']', '[');
        for c in s.chars() {
            if let Some(&open) = pairs.get(&c) {
                if let Some(prev) = stack.pop() {
                    if open == prev {
                        continue;
                    }
                }
                return false;
            } else {
                stack.push(c)
            }
        }
        stack.is_empty()
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::is_valid;
}
