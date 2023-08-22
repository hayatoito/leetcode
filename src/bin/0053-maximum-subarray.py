# @leetup=info id=53 lang=python3 slug=maximum-subarray



# @leetup=code



class Solution:
    # def maxSubArray(self, nums: list[int]) -> int:

    #     @cache
    #     def doit(i: int, picked: bool) -> int:
    #         if i == len(nums):
    #             return 0 if picked else -inf
    #         if picked:
    #             # end or continue to pick
    #             return max(0, doit(i + 1, True) + nums[i])
    #         # skip or pick
    #         return max(doit(i + 1, False), doit(i + 1, True) + nums[i])

    #     return doit(0, False)

    def maxSubArray(self, nums: list[int]) -> int:
        N = len(nums)
        assert N
        dp = [0] * N
        dp[0] = nums[0]

        for i in range(1, N):
            dp[i] = max(nums[i], dp[i - 1] + nums[i])

        return max(dp)


# @leetup=code
