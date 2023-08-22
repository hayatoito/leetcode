// @leetup=info id=169 lang=rust slug=majority-element

// @leetup=code

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut res = -1;
        for n in nums {
            if n == res {
                cnt += 1;
            } else if cnt > 1 {
                cnt -= 1;
            } else {
                cnt = 1;
                res = n;
            }
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::majority_element;
}
