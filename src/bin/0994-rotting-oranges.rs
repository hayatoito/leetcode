// @leetup=info id=994 lang=rust slug=rotting-oranges

// @leetup=code
use std::collections::*;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let r_len = grid.len();
        let c_len = grid[0].len();
        let mut q = VecDeque::new();
        let mut fresh_cnt = 0;
        for r in 0..r_len {
            for c in 0..c_len {
                if grid[r][c] == 1 {
                    fresh_cnt += 1;
                } else if grid[r][c] == 2 {
                    q.push_back(((r, c), 0));
                }
            }
        }

        let mut visited = HashSet::new();
        let mut res = 0;

        while let Some(((r, c), dist)) = q.pop_front() {
            if r < r_len && c < c_len && visited.insert((r, c)) {
                if grid[r][c] == 0 {
                    continue;
                }
                if grid[r][c] == 1 {
                    // grid[r][c] = 2;
                    fresh_cnt -= 1;
                    res = dist;
                }
                q.push_back(((r, c + 1), dist + 1));
                q.push_back(((r + 1, c), dist + 1));
                q.push_back(((r, c.wrapping_sub(1)), dist + 1));
                q.push_back(((r.wrapping_sub(1), c), dist + 1));
            }
        }

        if fresh_cnt == 0 {
            res
        } else {
            -1
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::oranges_rotting;
}
