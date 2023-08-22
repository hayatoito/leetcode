// @leetup=info id=227 lang=rust slug=basic-calculator-ii

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn calculate(mut s: String) -> i32 {
        let mut nums = Vec::new();
        // Append "+" as sentinel
        s.push_str("+");
        let mut num = 0;
        // previous operation
        let mut op = '+';
        for c in s.chars() {
            if let Some(d) = c.to_digit(10) {
                num = num * 10 + (d as i32)
            } else if !c.is_whitespace() {
                match op {
                    '+' => nums.push(num),
                    '-' => nums.push(-num),
                    '*' => *nums.last_mut().unwrap() *= num,
                    '/' => *nums.last_mut().unwrap() /= num,
                    _ => unreachable!()
                }
                op = c;
                num = 0;
            }
        }
        nums.iter().sum()
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::calculate;
}
