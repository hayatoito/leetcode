// @leetup=info id=50 lang=rust slug=powx-n

// @leetup=code

impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        let mut m = (n as i64).abs();
        let mut res = 1.0;
        while m != 0 {
            if m & 1 == 1 {
                res *= x;
            }
            x *= x;
            m >>= 1;
        }
        if n >= 0 {
            res
        } else {
            1.0 / res
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::my_pow;
}
