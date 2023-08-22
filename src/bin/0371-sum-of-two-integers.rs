// @leetup=info id=371 lang=rust slug=sum-of-two-integers

// @leetup=code

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            // (a, b) = (a ^ b, (a & b) << 1)
            let (a1, b1) = (a ^ b, (a & b) << 1);
            a = a1;
            b = b1;
        }
        a
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::get_sum;
}
