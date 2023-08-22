// @leetup=info id=329 lang=rust slug=longest-increasing-path-in-a-matrix

// @leetup=code

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut cache = vec![vec![None; matrix[0].len()]; matrix.len()];
        let mut res = 0;
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                res = res.max(dfs(-1, r, c, &matrix, &mut cache));
            }
        }
        res
    }
}

fn dfs(pre: i32, r: usize, c: usize, m: &[Vec<i32>], cache: &mut Vec<Vec<Option<i32>>>) -> i32 {
    if !(r < m.len() && c < m[0].len() && pre < m[r][c]) {
        return 0;
    }
    if let Some(n) = cache[r][c].as_ref() {
        return *n;
    }
    let mut res = 0;
    for (dr, dc) in [(0, 1), (0, !0), (1, 0), (!0, 0)] {
        let nr = r + dr;
        let nc = c + dc;
        res = res.max(1 + dfs(m[r][c], nr, nc, m, cache));
    }
    cache[r][c] = Some(res);
    res
}

// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::longest_increasing_path;
}
