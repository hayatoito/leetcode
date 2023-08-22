// @leetup=info id=14 lang=rust slug=longest-common-prefix

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter().reduce(|x, y| {
            x.chars().zip(y.chars()).take_while(|(a, b)| a == b).map(|a| a.0).collect()
        }).unwrap()
    }

    // pub fn longest_common_prefix(strs: Vec<String>) -> String {
    //     let s0 = strs[0].clone();
    //     let strs = strs[1..]
    //         .into_iter()
    //         .map(|s| s.chars().collect::<Vec<char>>())
    //         .collect::<Vec<_>>();

    //     let mut res = String::new();
    //     for (i, c) in s0.chars().enumerate() {
    //         for s in &strs {
    //             if i >= s.len() || s[i] != c {
    //                 return res;
    //             }
    //         }
    //         res.push(c);
    //     }
    //     res
    // }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::longest_common_prefix;
}
