class Solution:
    def minSubArrayLen(self, target: int, nums: list[int]) -> int:
        start = 0
        subarray_sum = 0
        ans = inf
        for index, n in enumerate(nums):
            subarray_sum += n
            while subarray_sum >= target:
                ans = min(ans, index + 1 - start)
                subarray_sum -= nums[start]
                start += 1

        return ans if ans != inf else 0
