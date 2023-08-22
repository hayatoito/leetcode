class Solution:
    def search(self, nums: list[int], target: int) -> int:

        left = 0
        right = len(nums)

        while left < right:
            mid = (left + right) // 2
            if nums[mid] == target:
                return mid
            if nums[mid] < target:
                left = mid + 1
            else:
                right = mid
        return -1
