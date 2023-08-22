// @leetup=info id=417 lang=rust slug=pacific-atlantic-water-flow

// @leetup=code

use std::collections::*;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let r_len = heights.len();
        let c_len = heights[0].len();
        let solve = Solve {
            heights,
            r_len,
            c_len,
        };
        solve.solve()
    }
}

struct Solve {
    heights: Vec<Vec<i32>>,
    r_len: usize,
    c_len: usize,
}

impl Solve {
    fn solve(&self) -> Vec<Vec<i32>> {
        let mut q = VecDeque::new();
        for r in 0..self.r_len {
            q.push_back((r, 0));
        }
        for c in 0..self.c_len {
            q.push_back((0, c));
        }
        let visited1 = self.bfs(q);

        let mut q = VecDeque::new();
        for r in 0..self.r_len {
            q.push_back((r, self.c_len - 1));
        }
        for c in 0..self.c_len {
            q.push_back((self.r_len - 1, c));
        }
        let visited2 = self.bfs(q);

        visited1
            .intersection(&visited2)
            .into_iter()
            .copied()
            .map(|(r, c)| vec![r as i32, c as i32])
            .collect()
    }

    fn bfs(&self, mut q: VecDeque<(usize, usize)>) -> HashSet<(usize, usize)> {
        let mut visited = HashSet::new();
        while let Some((r, c)) = q.pop_front() {
            if !visited.insert((r, c)) {
                continue;
            }
            for (dr, dc) in [(0, 1), (0, !0), (1, 0), (!0, 0)] {
                let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
                if nr < self.r_len && nc < self.c_len && self.heights[r][c] <= self.heights[nr][nc]
                {
                    q.push_back((nr, nc));
                }
            }
        }
        visited
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::pacific_atlantic;
}
