# @leetup=info id=223 lang=python3 slug=rectangle-area



# @leetup=code
from typing import Tuple


class Solution:
    def computeArea(
        self,
        ax1: int,
        ay1: int,
        ax2: int,
        ay2: int,
        bx1: int,
        by1: int,
        bx2: int,
        by2: int,
    ) -> int:
        def interval_intersection(seg1: Tuple[int, int], seg2: Tuple[int, int]) -> int:
            left = max(seg1[0], seg2[0])
            right = min(seg1[1], seg2[1])
            return (right - left) if left < right else 0

        area_a = (ax2 - ax1) * (ay2 - ay1)
        area_b = (bx2 - bx1) * (by2 - by1)
        seg1 = interval_intersection((ax1, ax2), (bx1, bx2))
        seg2 = interval_intersection((ay1, ay2), (by1, by2))
        area_intersection = seg1 * seg2
        return area_a + area_b - area_intersection


# @leetup=code
