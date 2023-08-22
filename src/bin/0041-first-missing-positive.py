# @leetup=info id=41 lang=python3 slug=first-missing-positive





# @leetup=code


class Solution:
    def firstMissingPositive(self, nums: list[int]) -> int:
        # Use input array `nums` as memory.

        for i, n in enumerate(nums):
            if n < 0:
                nums[i] = 0

        print("1. f{nums=}")

        for i, n in enumerate(nums):
            val = abs(n)
            if 1 <= val <= len(nums):
                if nums[val - 1] > 0:
                    nums[val - 1] *= -1
                elif nums[val - 1] == 0:
                    nums[val - 1] = -1 * (len(nums) + 1)

        print("2. f{nums=}")

        for i in range(1, len(nums) + 1):
            if nums[i - 1] >= 0:
                return i
        return len(nums) + 1


# @leetup=code
