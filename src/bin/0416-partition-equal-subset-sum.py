# @leetup=info id=416 lang=python3 slug=partition-equal-subset-sum



# @leetup=code
class Solution:
    def canPartition(self, nums: list[int]) -> bool:
        N = sum(nums) + 1
        dp = [False] * N
        dp[0] = True

        for n in nums:
            next_dp = [False] * N
            for m, possible in enumerate(dp):
                if not possible:
                    continue
                next_dp[abs(m + n)] = True
                next_dp[abs(m - n)] = True
            dp = next_dp

        return dp[0]


# @leetup=code
