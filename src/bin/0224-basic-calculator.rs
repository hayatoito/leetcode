// @leetup=info id=224 lang=rust slug=basic-calculator


// Ref: https://leetcode.com/problems/basic-calculator/solutions/62361/iterative-java-solution-with-stack/

// @leetup=code
#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut res = 0;
        let mut res_stack = Vec::new();
        let mut sign_stack = Vec::new();
        let mut sign: i32 = 1;
        let mut num: u32 = 0;
        for c in s.chars() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap();
            } else if c == '+' {
                res += sign * (num as i32);
                num = 0;
                sign = 1;
            } else if c == '-' {
                res += sign * (num as i32);
                num = 0;
                sign = -1;
            } else if c == '(' {
                assert_eq!(num, 0);
                res_stack.push(res);
                sign_stack.push(sign);
                res = 0;
                sign = 1;
            } else if c == ')' {
                res += sign * (num as i32);

                res *= sign_stack.pop().unwrap();
                res += res_stack.pop().unwrap();

                num = 0;
                sign = 1;
            }
        }
        res + sign * (num as i32)
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::calculate;
}
