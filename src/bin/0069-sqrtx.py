# @leetup=info id=69 lang=python3 slug=sqrtx



# @leetup=code


class Solution:
    def mySqrt(self, x: int) -> int:
        if x == 0:
            return 0
        left = 0
        right = x + 1
        while left < right:
            m = left + (right - left) // 2
            if m * m <= x:
                left = m + 1
            else:
                right = m
        return left - 1


# @leetup=code
