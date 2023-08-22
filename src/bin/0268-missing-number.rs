// @leetup=info id=268 lang=rust slug=missing-number

// @leetup=code
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        for (i, n) in nums.into_iter().enumerate() {
            a = a ^ (i as i32 + 1) ^ n;
        }
        a
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::missing_number;
}
