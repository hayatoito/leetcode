# @leetup=info id=55 lang=python3 slug=jump-game



# @leetup=code
class Solution:
    def canJump(self, nums: list[int]) -> bool:
        right = nums[0]
        for left, n in enumerate(nums):
            if right >= len(nums) - 1:
                return True
            right = max(right, left + n)
            if left == right:
                return False

        return True


# @leetup=code
