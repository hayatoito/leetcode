// @leetup=info id=118 lang=rust slug=pascals-triangle

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut res = Vec::new();
        res.push(vec![1]);
        for i in 1..num_rows {
            let mut row = vec![1];
            for j in 1..i {
                row.push(res[i - 1][j-1] + res[i - 1][j]);
            }
            row.push(1);
            res.push(row);
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::generate;
}
