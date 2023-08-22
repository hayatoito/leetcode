# @leetup=info id=739 lang=python3 slug=daily-temperatures



from typing import Optional

# @leetup=code
class Solution:
    def dailyTemperatures(self, temperatures: list[int]) -> list[int]:
        N = len(temperatures)
        res = [0] * N
        stack = []  # holds index.
        for i, n in enumerate(temperatures):
            while stack and temperatures[stack[-1]] < n:
                prev_i = stack.pop()
                res[prev_i] = i - prev_i
            stack.append(i)
        return res


# @leetup=code
