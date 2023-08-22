// @leetup=info id=70 lang=rust slug=climbing-stairs

// @leetup=code
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp0 = 1;
        let mut dp1 = 1;
        for _ in 2..n + 1 {
            // [2023-04-13 Thu] leetcode's rust doesn't support this:
            // destructuring assignments are unstable (solution.rs)
            // (dp1, dp0) = (dp0 + dp1, dp1);

            let tmp = dp0 + dp1;
            dp0 = dp1;
            dp1 = tmp;
        }
        dp1
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::climb_stairs;
}
