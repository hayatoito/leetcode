# @leetup=info id=11 lang=python3 slug=container-with-most-water



# @leetup=code
class Solution:
    def maxArea(self, height: list[int]) -> int:
        res = 0
        left = 0
        right = len(height) - 1
        while left < right:
            res = max(res, min(height[left], height[right]) * (right - left))
            if height[left] < height[right]:
                left += 1
            else:
                right -= 1
        return res


# @leetup=code
