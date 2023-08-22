// @leetup=info id=66 lang=rust slug=plus-one

// @leetup=code

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();
        for i in 0..digits.len() {
            digits[i] += 1;
            if digits[i] < 10 {
                break;
            }
            digits[i] = 0;
            if i == digits.len() - 1 {
                digits.push(1);
            }
        }
        digits.reverse();
        digits
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::plus_one;
}
