# @leetup=info id=322 lang=python3 slug=coin-change



# @leetup=code


class Solution:
    def coinChange(self, coins: list[int], amount: int) -> int:
        dp = [inf] * (amount + 1)
        dp[0] = 0

        for a in range(1, amount + 1):
            for coin in coins:
                # if a - coin >= 0 and dp[a-coin] != inf:
                if a - coin >= 0:
                    dp[a] = min(dp[a], dp[a - coin] + 1)

        return dp[amount] if dp[amount] != inf else -1


# @leetup=code
