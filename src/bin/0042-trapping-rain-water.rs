// @leetup=info id=42 lang=rust slug=trapping-rain-water

// @leetup=code
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // Monotonic stack (descending height[index] order).
        // height[stack[i]] > height[stack[i + 1]] > ....
        let mut stack = Vec::new();
        let mut res = 0;

        for (i, &n) in height.iter().enumerate() {
            // while !stack.is_empty() && height[*stack.last().unwrap()] < n {
            while let Some(true) = stack.last().map(|i| height[*i] < n) {
                let prev_i = stack.pop().unwrap();
                let h = height[prev_i];

                if let Some(&left_index) = stack.last() {
                    let left_height = height[left_index];
                    // rectanble:
                    //
                    // left_index + 1
                    //    |                       i
                    //    |                       |
                    //    |                       |
                    //    -------------------------  h (= height[prev_i])
                    res += (n.min(left_height) - h) * (i - left_index - 1) as i32
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
    let _ = Solution::trap;
}
