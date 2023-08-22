# @leetup=info id=42 lang=python3 slug=trapping-rain-water



from typing import Optional

# @leetup=code
class Solution:
    def trap(self, height: list[int]) -> int:
        # Monotonic stack (descending height[index] order).
        # height[stack[i]] > height[stack[i + 1]] > ....
        stack = []
        res = 0

        for i, n in enumerate(height):
            while stack and height[stack[-1]] < n:
                prev_i = stack.pop()
                h = height[prev_i]

                if stack:
                    left_index = stack[-1]
                    left_height = height[left_index]
                    # rectanble:
                    #
                    # left_index + 1
                    #    |                       i
                    #    |                       |
                    #    |                       |
                    #    -------------------------  h (= height[prev_i])
                    #
                    res += (min(n, left_height) - h) * (i - left_index - 1)
            stack.append(i)
        return res


# @leetup=code
