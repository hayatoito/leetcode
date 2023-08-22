// @leetup=info id=29 lang=rust slug=divide-two-integers

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == divisor {
            return 1;
        }
        let is_negative = (dividend < 0) ^ (divisor < 0);
        let p = Solution::divide1(Solution::abs(dividend), Solution::abs(divisor));
        if is_negative {
            if p == 1 << 31 {
                i32::MIN
            } else {
                -(p as i32)
            }
        } else {
            if p == 1 << 31 {
                i32::MAX
            } else {
                p as i32
            }
        }
    }

    fn abs(a: i32) -> u32 {
        if a == i32::MIN {
            1 << 31
        } else {
            a.abs() as u32
        }
    }

    fn divide1(mut a: u32, b: u32) -> u32 {
        let mut res = 0;
        while a >= b {
            let mut i = 0;
            while a > (b << (i + 1)) {
                i += 1;
            }
            res += 1 << i;
            a -= b << i;
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    println!("{}", Solution::divide(-2147483648, -1));
    println!("{}", Solution::divide(-2147483648, 1));
    println!("{}", Solution::divide(-2147483648, -2147483648));
}
