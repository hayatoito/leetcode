// @leetup=info id=13 lang=rust slug=roman-to-integer

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    // Ref: https://leetcode.com/problems/roman-to-integer/solutions/374718/rust-pattern-matching-without-extra-allocation/
    pub fn roman_to_int(s: String) -> i32 {
        s.chars().rfold(0, |acc, c| {
            acc + match c {
                'I' if acc >= 5 => -1,
                'I' => 1,
                'V' => 5,
                'X' if acc >= 50 => -10,
                'X' => 10,
                'L' => 50,
                'C' if acc >= 500 => -100,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => unreachable!()
            }
        })
    }

    // pub fn roman_to_int(s: String) -> i32 {
    //     let rules = vec![
    //         ("M", 1000),
    //         ("CM", 900),
    //         ("D", 500),
    //         ("CD", 400),
    //         ("C", 100),
    //         ("XC", 90),
    //         ("L", 50),
    //         ("XL", 40),
    //         ("X", 10),
    //         ("IX", 9),
    //         ("V", 5),
    //         ("IV", 4),
    //         ("I", 1),
    //     ];

    //     fn doit(s: &str, rules: &[(&str, i32)]) -> i32 {
    //         if s.is_empty() {
    //             return 0;
    //         }
    //         for (prefix, a) in rules {
    //             if s.starts_with(prefix) {
    //                 return a + doit(&s[prefix.len()..], rules);
    //             }
    //         }
    //         unreachable!();
    //     }

    //     doit(&s, &rules)
    // }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::roman_to_int;
}
