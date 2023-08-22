# @leetup=info id=198 lang=python3 slug=house-robber



# @leetup=code
class Solution:
    def rob(self, nums: list[int]) -> int:
        take = 0
        not_take = 0
        for n in nums:
            take, not_take = (not_take + n, max(take, not_take))

        return max(take, not_take)


# @leetup=code
