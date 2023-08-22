# @leetup=info id=84 lang=python3 slug=largest-rectangle-in-histogram




from typing import Optional

# @leetup=code
class Solution:
    def largestRectangleArea(self, heights: list[int]) -> int:
        # rectable:
        # area = current_bar_height * (rightmost_bar_index - leftmost_bar_index + 1)
        # all(height[i] >= current_bar_height for i in range(current_index...(rightmost_bar_index + 1)))

        # Monotonic stack (Stores index). Increasing.
        # height[i] < height[i + 1]]

        # Sentinel
        heights = [0] + heights + [0]

        stack = []
        res = 0
        for i, n in enumerate(heights):
            while stack and heights[stack[-1]] > n:
                prev_i = stack.pop()
                h = heights[prev_i]
                if stack:
                    w = i - stack[-1] - 1
                    area = h * w
                    res = max(res, area)
            stack.append(i)
        return res


# @leetup=code
