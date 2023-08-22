// @leetup=info id=131 lang=rust slug=palindrome-partitioning

// @leetup=code

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let solver = Solver::new(s.chars().collect());
        solver.solve()
    }
}

struct Solver {
    s: Vec<char>,
    is_palindrome: Vec<Vec<bool>>,
    res: Vec<Vec<String>>,
}

impl Solver {
    fn new(s: Vec<char>) -> Self {
        let len = s.len();
        Solver {
            s,
            is_palindrome: vec![vec![false; len + 1]; len + 1],
            res: Vec::new(),
        }
    }

    fn solve(mut self) -> Vec<Vec<String>> {
        self.fill_is_palindrome();
        self.dfs(0, &mut Vec::new());
        self.res
    }

    fn dfs(&mut self, pos: usize, v: &mut Vec<String>) {
        if pos == self.s.len() {
            self.res.push(v.clone());
            return;
        }
        for i in pos + 1..self.s.len() + 1 {
            if self.is_palindrome[pos][i] {
                v.push(self.s[pos..i].iter().collect());
                self.dfs(i, v);
                v.pop().unwrap();
            }
        }
    }

    fn fill_is_palindrome(&mut self) {
        for i in 0..self.s.len() {
            self.fill(i, i + 1);
            self.fill(i, i + 2);
        }
    }

    fn fill(&mut self, mut left: usize, mut right: usize) {
        while left < self.s.len() && right < self.s.len() + 1 && self.s[left] == self.s[right - 1] {
            self.is_palindrome[left][right] = true;
            left = left.wrapping_sub(1);
            right += 1;
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::partition;
}
