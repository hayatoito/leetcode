# @leetup=info id=153 lang=python3 slug=find-minimum-in-rotated-sorted-array



# @leetup=code
class Solution:
    def findMin(self, nums: list[int]) -> int:
        N = len(nums)
        if N == 1:
            return nums[0]
        if nums[0] < nums[-1]:
            # Not rotated
            return nums[0]
        left = 0
        right = N

        while left < right:
            mid = (left + right) // 2
            if nums[0] <= nums[mid]:
                left = mid + 1
            else:
                right = mid

        return nums[left]


# @leetup=code
