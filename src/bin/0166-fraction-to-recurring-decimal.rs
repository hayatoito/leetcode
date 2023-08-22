// @leetup=info id=166 lang=rust slug=fraction-to-recurring-decimal

// @leetup=code

use std::collections::*;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }
        let sign = numerator.signum() * denominator.signum();
        let mut digits: Vec<u64> = vec![];
        let mut n = (numerator as i64).abs() as u64;
        let d = (denominator as i64).abs() as u64;
        let mut cache = HashMap::new();

        while n != 0 {
            if let Some(start) = cache.get(&n) {
                return Solution::gen(&digits, sign, Some(*start));
            }
            if digits.len() > 0 {
                cache.insert(n, digits.len());
            }
            if n > d {
                digits.push(n / d);
                n = n % d * 10;
            } else {
                digits.push(0);
                n *= 10;
            }
        }
        Solution::gen(&digits, sign, None)
    }

    fn gen(digits: &[u64], sign: i32, repeat_start: Option<usize>) -> String {
        let mut s = String::new();
        if sign == -1 {
            s.push_str("-");
        }
        for i in 0..repeat_start.unwrap_or(digits.len()) {
            s.push_str(&digits[i].to_string());
            if i == 0 && digits.len() > 1 {
                s.push_str(".");
            }
        }
        if let Some(repeat_start) = repeat_start {
            s.push_str("(");
            for i in repeat_start..digits.len() {
                s.push_str(&digits[i].to_string());
            }
            s.push_str(")");
        }
        return s;
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::fraction_to_decimal;
}
