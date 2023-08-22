# @leetup=info id=268 lang=python3 slug=missing-number



# @leetup=code
class Solution:
    def missingNumber(self, nums: list[int]) -> int:
        a = 0
        for i, n in enumerate(nums):
            a = a ^ n ^ (i + 1)
        return a


# @leetup=code
