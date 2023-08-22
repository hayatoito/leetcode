// @leetup=info id=198 lang=rust slug=house-robber

// @leetup=code

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut not_take = 0;
        let mut take = 0;
        for n in nums {
            // (take, not_take) = (not_take + n, take.max(not_take));
            let take_next = not_take + n;
            not_take = take.max(not_take);
            take = take_next;
        }
        not_take.max(take)
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::rob;
}
