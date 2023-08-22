# @leetup=info id=217 lang=python3 slug=contains-duplicate



# @leetup=code


class Solution:
    def containsDuplicate(self, nums: list[int]) -> bool:
        return len(nums) != len(set(nums))


# @leetup=code
