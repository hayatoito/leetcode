class Solution:
    def moveZeroes(self, nums: list[int]):
        left = 0
        right = 0

        # invariant
        # nums[:left] = non-zero
        # nums[left:right] = zero

        # [1, 3, 0, 0, 12]
        #       left   right

        while right < len(nums):
            if nums[right] == 0:
                right += 1
            else:
                nums[left], nums[right] = nums[right], nums[left]
                left += 1
                right += 1
