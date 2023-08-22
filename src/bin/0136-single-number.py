# @leetup=info id=136 lang=python3 slug=single-number



# @leetup=code
class Solution:
    def singleNumber(self, nums: list[int]) -> int:
        a = 0
        for n in nums:
            a ^= n
        return a


# @leetup=code
