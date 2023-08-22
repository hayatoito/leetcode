// @leetup=info id=435 lang=rust slug=non-overlapping-intervals

// @leetup=code
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort();
        let mut res = 0;
        let mut end = intervals[0][1];
        for interval in intervals.into_iter().skip(1) {
            if interval[0] < end {
                res += 1;
                end = end.min(interval[1])
            } else {
                end = interval[1]
            }
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::erase_overlap_intervals;
}
