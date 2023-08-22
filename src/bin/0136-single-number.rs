// @leetup=info id=136 lang=rust slug=single-number

// @leetup=code
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, a| acc ^ a)
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::single_number;
}
