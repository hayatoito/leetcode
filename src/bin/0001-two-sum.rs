// @leetup=custom
// @leetup=info id=1 lang=rust slug=two-sum

// @leetup=custom

// @leetup=code
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut visited = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            let m = target - n;
            if let Some(&j) = visited.get(&m) {
                return vec![j, i as i32];
            }
            visited.insert(n, i as i32);
        }
        unreachable!()
    }
}
// @leetup=code

// @leetup=inject:after_code
struct Solution;

fn main() {
    let _func = Solution::two_sum;

    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test0() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

// @leetup=inject:after_code
