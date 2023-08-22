// @leetup=info id=130 lang=rust slug=surrounded-regions

// @leetup=code

use std::collections::*;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if board[r][c] == 'X' {
                    continue;
                }
                let mut q = VecDeque::new();
                let mut visited = HashSet::new();
                let mut escaped = false;
                q.push_back((r, c));

                while let Some((r, c)) = q.pop_front() {
                    if !(r < board.len() && c < board[0].len()) {
                        escaped = true;
                        continue;
                    }
                    if board[r][c] != 'O' {
                        continue;
                    }
                    if !visited.insert((r, c)) {
                        continue;
                    }
                    board[r][c] = 'X';
                    for (dr, dc) in [(0, 1), (0, !0), (1, 0), (!0, 0)] {
                        q.push_back((r.wrapping_add(dr), c.wrapping_add(dc)));
                    }
                }
                if escaped {
                    for (r, c) in visited {
                        board[r][c] = 'O';
                    }
                }
            }
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::solve;
}
