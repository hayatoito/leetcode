// @leetup=info id=334 lang=rust slug=increasing-triplet-subsequence

// @leetup=code

use std::cmp::Ordering;

impl Solution {

    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut single: Option<i32> = None;
        let mut double: Option<i32> = None;
        for n in nums {
            match (single, double)  {
                (None, None) => {
                    single = Some(n);
                },
                (Some(a), None) => {
                    match a.cmp(&n) {
                        Ordering::Less => {
                            double = Some(n);
                        }
                        Ordering::Equal => {},
                        Ordering::Greater => {
                            single = Some(n);
                        },
                    }
                },
                (Some(a), Some(b)) => {
                    if b < n {
                        return true;
                    }
                    match a.cmp(&n) {
                        Ordering::Less => {
                            if n < b {
                                double = Some(n);
                            }
                        }
                        Ordering::Equal => {},
                        Ordering::Greater => {
                            single = Some(n);
                        },
                    }
                }
                (None, Some(_)) => unreachable!()
            }

        }
        false
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::increasing_triplet;
}
