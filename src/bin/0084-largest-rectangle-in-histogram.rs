// @leetup=info id=84 lang=rust slug=largest-rectangle-in-histogram


// Ref: https://leetcode.com/problems/largest-rectangle-in-histogram/solutions/28917/ac-python-clean-solution-using-stack-76ms/
// @leetup=code
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let heights = {
            let mut h = vec![0];
            h.extend(heights.into_iter());
            h.push(0);
            h
        };
        let mut stack = Vec::new();
        let mut res = 0;
        for (i, &n) in heights.iter().enumerate() {
            while let Some(true) = stack.last().map(|i| heights[*i] > n) {
                let prev_i = stack.pop().unwrap();
                let h = heights[prev_i];
                if let Some(&left_index) = stack.last() {
                    //      |-----|
                    //      |     |--|
                    //   |--|     |  |
                    //   |  |     |  |
                    //            ^ i
                    //          ^ prev_i
                    //   ^ left_index

                    // for i in (left_index + 1..prev_i + 1) {
                    //    assert_eq!(heigt[i] >= height[prev_i])
                    //
                    let width = i - (left_index + 1);

                    let area = h * width as i32;
                    res = res.max(area);
                }
            }
            stack.push(i);
        }
        res

    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::largest_rectangle_area;
}
