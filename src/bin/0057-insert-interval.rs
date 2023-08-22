// @leetup=info id=57 lang=rust slug=insert-interval

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {

        fn is_overlap(i1: &[i32], i2: &[i32]) -> bool {
            i1[0] <= i2[1] && i2[0] <= i1[1]
        }

        fn merge_overlap(i1: &[i32], i2: &[i32]) -> Vec<i32> {
            vec![i1[0].min(i2[0]), i1[1].max(i2[1])]
        }

        let mut left = Vec::new();
        let mut right = Vec::new();

        for interval in intervals {
            if is_overlap(&interval, &new_interval) {
                new_interval = merge_overlap(&interval, &new_interval);
            } else if interval[1] < new_interval[0] {
                left.push(interval);
            } else {
                right.push(interval);
            }
        }

        left.push(new_interval);
        left.append(&mut right);
        left
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::insert;
}
