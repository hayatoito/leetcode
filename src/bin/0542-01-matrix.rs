// @leetup=info id=542 lang=rust slug=01-matrix

// @leetup=code
use std::collections::*;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let r_len = mat.len();
        let c_len = mat[0].len();

        let mut res = vec![vec![-1; c_len]; r_len];

        let mut q = VecDeque::new();
        for r in 0..r_len {
            for c in 0..c_len {
                if mat[r][c] == 0 {
                    q.push_back(((r, c), 0));
                }
            }
        }

        while let Some(((r, c), dist)) = q.pop_front() {
            if r < r_len && c < c_len {
                if res[r][c] == -1 {
                    res[r][c] = dist;
                    q.push_back(((r, c + 1), dist + 1));
                    q.push_back(((r + 1, c), dist + 1));
                    q.push_back(((r, c.wrapping_sub(1)), dist + 1));
                    q.push_back(((r.wrapping_sub(1), c), dist + 1));
                }
            }
        }

        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::update_matrix;
}
