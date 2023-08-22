// @leetup=info id=10 lang=rust slug=regular-expression-matching

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Solver::new(s, p).is_match(0, 0)
    }
}

struct Solver {
    s: Vec<char>,
    p: Vec<char>,
    cache: HashMap<(usize, usize), bool>,
}

impl Solver {
    fn new(s: String, p: String) -> Self {
        Self {
            s: s.chars().collect(),
            p: p.chars().collect(),
            cache: Default::default(),
        }
    }

    fn is_match(&mut self, i: usize, j: usize) -> bool {
        if i == self.s.len() && j == self.p.len() {
            return true;
        }
        if j == self.p.len() {
            return false;
        }
        if let Some(b) = self.cache.get(&(i, j)) {
            return *b;
        }
        let mut ok = false;
        let c = self.p[j];
        if j + 1 < self.p.len() && self.p[j + 1] == '*' {
            ok = self.is_match(i, j + 2);
            for k in i..self.s.len() {
                if c == '.' || self.s[k] == c {
                    ok = ok || self.is_match(k + 1, j + 2);
                } else {
                    break;
                }
            }
        } else if i < self.s.len() {
            if c == '.' || self.s[i] == c {
                ok = self.is_match(i + 1, j + 1);
            }
        }
        self.cache.insert((i, j), ok);
        ok
    }
}

// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::is_match;
}
