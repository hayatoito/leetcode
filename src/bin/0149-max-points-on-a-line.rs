// @leetup=info id=149 lang=rust slug=max-points-on-a-line

// @leetup=code

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// Ref: https://stackoverflow.com/questions/39638363/how-can-i-use-a-hashmap-with-f64-as-key-in-rust
struct Distance(f64);

impl Distance {
    fn canonicalize(&self) -> i64 {
        (self.0 * 1024.0 * 1024.0 * 1024.0).round() as i64
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Distance) -> bool {
        self.canonicalize() == other.canonicalize()
    }
}

impl Eq for Distance {}

impl Hash for Distance {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.canonicalize().hash(state);
    }
}

impl Solution {
    // O(n^2)
    // https://leetcode.com/problems/max-points-on-a-line/editorial/?envType=featured-list&envId=top-interview-questions
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 3 {
            return points.len() as i32;
        }
        let mut res = 0;
        for i in 0..points.len() {
            let x1 = points[i][0] as f64;
            let y1 = points[i][1] as f64;
            let mut cnt: HashMap<Distance, i32> = HashMap::new();
            for j in 0..points.len() {
                if i == j {
                    continue;
                }
                let x2 = points[j][0] as f64;
                let y2 = points[j][1] as f64;

                let atan = Distance(f64::atan2(y2 - y1, x2 - x1));
                *cnt.entry(atan).or_insert(0) += 1;
            }
            res = res.max(cnt.into_values().max().unwrap());
        }
        res + 1
    }

    // O(n^3)
    /*
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 3 {
            return points.len() as i32;
        }
        let mut res = 0;
        for i in 0..points.len() {
            let x1 = points[i][0];
            let y1 = points[i][1];
            for j in i + 1..points.len() {
                let mut cnt = 2;
                let x2 = points[j][0];
                let y2 = points[j][1];
                for k in j + 1..points.len() {
                    let x3 = points[k][0];
                    let y3 = points[k][1];
                    // https://math.stackexchange.com/questions/701862/how-to-find-if-the-points-fall-in-a-straight-line-or-not
                    if (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1) == 0 {
                        cnt += 1
                    }
                }
                res = res.max(cnt);
            }
        }
        res
    }
    */
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::max_points;
}
