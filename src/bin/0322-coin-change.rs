// @leetup=info id=322 lang=rust slug=coin-change

// @leetup=code
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![i32::MAX; amount + 1];
        dp[0] = 0;
        for a in 1..amount + 1 {
            for coin in &coins {
                let coin = *coin as usize;
                if coin <= a {
                    dp[a] = dp[a].min(dp[(a - coin)].saturating_add(1));
                }
            }
        }
        if dp[amount] == i32::MAX {
            -1
        } else {
            dp[amount]
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::coin_change;
}
