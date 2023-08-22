import bisect

class Solution:
    def search(self, nums: list[int], target: int) -> int:

        def find_min_index():
            N = len(nums)
            if N == 1:
                return 0
            if nums[0] < nums[-1]:
                return 0
            left = 0
            right = N
            while left < right:
                mid = (left + right) // 2
                if nums[0] <= nums[mid]:
                    left = mid + 1
                else:
                    right = mid

            return left

        min_index = find_min_index()

        # left sorted array
        index = bisect.bisect_left(nums, target, hi=min_index)
        if index < min_index and nums[index] == target:
            return index
        # right sorted array
        index= bisect.bisect_left(nums, target, lo=min_index)
        if index < len(nums) and nums[index] == target:
            return index
        return -1
