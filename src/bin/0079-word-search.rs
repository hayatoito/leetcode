// @leetup=info id=79 lang=rust slug=word-search

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;
use std::num::Wrapping;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<_>>();
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if Solution::dfs(Wrapping(r), Wrapping(c), &word[..], &mut board) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        r: Wrapping<usize>,
        c: Wrapping<usize>,
        word: &[char],
        board: &mut Vec<Vec<char>>,
    ) -> bool {
        if !(r.0 < board.len() && c.0 < board[0].len() && board[r.0][c.0] == word[0]) {
            return false;
        }
        if word.len() == 1 {
            return true;
        }
        board[r.0][c.0] = '*';
        for (dr, dc) in [(0, 1), (0, !0), (1, 0), (!0, 0)] {
            if Self::dfs(r + Wrapping(dr), c + Wrapping(dc), &word[1..], board) {
                return true;
            }
        }
        board[r.0][c.0] = word[0];
        false
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::exist;
}
