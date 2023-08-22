# @leetup=info id=70 lang=python3 slug=climbing-stairs



# @leetup=code
class Solution:
    def climbStairs(self, n: int) -> int:
        # dp = [0] * (n + 1)
        # dp[0] = 1
        # dp[1] = 1
        # for i in range(2, n + 1):
        #     dp[i] = dp[i - 1] + dp[i - 2]
        # return dp[n]

        dp0 = 1
        dp1 = 1
        for _ in range(2, n + 1):
            (dp1, dp0) = (dp0 + dp1, dp1)
        return dp1


# @leetup=code
