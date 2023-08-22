# @leetup=info id=377 lang=python3 slug=combination-sum-iv



# @leetup=cod
class Solution:
    def combinationSum4(self, nums: list[int], target: int) -> int:
        dp = [0] * (target + 1)
        dp[0] = 1
        for i in range(target):
            for n in nums:
                if i + n <= target:
                    dp[i + n] += dp[i]
        return dp[target]


# @leetup=code
