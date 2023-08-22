// @leetup=info id=56 lang=rust slug=merge-intervals

// @leetup=code
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut res: Vec<Vec<i32>> = Vec::new();
        for interval in intervals {
            if let Some(true) = res.last().map(|pre| interval[0] <= pre[1]) {
                (*res.last_mut().unwrap())[1] = (*res.last().unwrap())[1].max(interval[1]);
            } else {
                res.push(interval.clone())
            }
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::merge;
}
