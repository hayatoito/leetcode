// @leetup=info id=739 lang=rust slug=daily-temperatures

// @leetup=code
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack = Vec::new(); // Holds index
        for (i, &n) in temperatures.iter().enumerate() {
            while let Some(&prev_i) = stack.last() {
                if temperatures[prev_i] < n {
                    stack.pop();
                    res[prev_i] = i - prev_i;
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        res.into_iter().map(|i| i as i32).collect()
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::daily_temperatures;
}
