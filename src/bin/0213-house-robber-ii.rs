// @leetup=info id=213 lang=rust slug=house-robber-ii

// @leetup=code
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let solve = Solution::solve;
        solve(&nums[1..])
            .max(solve(&nums[0..nums.len() - 1]))
            .max(nums[0])
    }

    fn solve(nums: &[i32]) -> i32 {
        let mut take = 0;
        let mut not_take = 0;
        for n in nums {
            // (take, not_take) = (not_take + n, take.max(not_take));
            let tmp = take;
            take = not_take + n;
            not_take = tmp.max(not_take);
        }
        take.max(not_take)
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::rob;
}
