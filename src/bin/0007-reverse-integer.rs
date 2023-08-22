// @leetup=info id=7 lang=rust slug=reverse-integer

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == i32::MIN {
            return 0;
        }
        let r = Solution::reverse1(x.abs() as u32);
        if x >= 0 {
            r as i32
        } else {
            -(r as i32)
        }
    }

    fn reverse1(mut x: u32) -> u32 {
        let mut r: u32 = 0;
        let umax = i32::MAX as u32;
        while x != 0 {
            if r > umax / 10 {
                return 0;
            }
            if umax - r * 10 < x % 10 {
                return 0;
            }
            r = r * 10 + x % 10;
            x /= 10;
        }
        r
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::reverse;
}
