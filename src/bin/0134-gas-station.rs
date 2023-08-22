// @leetup=info id=134 lang=rust slug=gas-station

// @leetup=code

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_gain = 0;
        let mut curr_gain = 0;
        let mut answer = 0;
        for i in 0..gas.len() {
            total_gain += gas[i] - cost[i];
            curr_gain += gas[i] - cost[i];
            if curr_gain < 0 {
                curr_gain = 0;
                answer = i + 1;
            }
        }
        if total_gain >= 0 {
            answer as i32
        } else {
            -1
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::can_complete_circuit;
}
