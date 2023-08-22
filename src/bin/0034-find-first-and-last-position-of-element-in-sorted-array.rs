// @leetup=info id=34 lang=rust slug=find-first-and-last-position-of-element-in-sorted-array

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

fn lower_bound(xs: &[i32], x: i32) -> usize {
    let mut left = 0;
    let mut right = xs.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if xs[mid] < x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn upper_bound(xs: &[i32], x: i32) -> usize {
    let mut left = 0;
    let mut right = xs.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if xs[mid] <= x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lower = lower_bound(&nums, target);
        let upper = upper_bound(&nums, target);
        if lower == upper {
            vec![-1, -1]
        } else {
            vec![lower as i32, (upper - 1) as i32]
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::search_range;
}
