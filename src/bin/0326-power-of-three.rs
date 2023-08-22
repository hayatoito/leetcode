// @leetup=info id=326 lang=rust slug=power-of-three

// @leetup=code

impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        while n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::is_power_of_three;
}
