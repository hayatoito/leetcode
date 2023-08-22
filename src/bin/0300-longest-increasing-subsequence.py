# @leetup=info id=300 lang=python3 slug=longest-increasing-subsequence



# @leetup=code
class Solution:
    def lengthOfLIS(self, nums: list[int]) -> int:
        dp = [1] * len(nums)
        for i in range(1, len(nums)):
            for pre in range(i):
                if nums[pre] < nums[i]:
                    dp[i] = max(dp[i], dp[pre] + 1)

        return max(dp)


# @leetup=code
