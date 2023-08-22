// @leetup=info id=69 lang=rust slug=sqrtx

// @leetup=code

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut left: i64 = 0;
        let mut right: i64 = x as i64 + 1;
        while left < right {
            let m = left + (right - left) / 2;
            if m * m <= x as i64 {
                left = m + 1;
            } else {
                right = m;
            }
        }
        (left - 1) as i32
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::my_sqrt;
}
