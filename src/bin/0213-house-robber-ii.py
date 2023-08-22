# @leetup=info id=213 lang=python3 slug=house-robber-ii



# @leetup=code


class Solution:
    def rob(self, nums: list[int]) -> int:
        def solve(nums: list[int]) -> int:
            take = 0
            not_take = 0
            for n in nums:
                take, not_take = (not_take + n, max(take, not_take))
            return max(take, not_take)

        return max(solve(nums[1:]), solve(nums[:-1]), nums[0])


# @leetup=code
