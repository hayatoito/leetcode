# @leetup=info id=4 lang=python3 slug=median-of-two-sorted-arrays



from typing import Optional

# @leetup=code

import math


class Solution:

    inf = math.inf

    def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
        a = nums1
        b = nums2

        if len(a) > len(b):
            # Let len(a) < len(b) since b_mid (= half - a_mid) should be >= 0
            a, b = b, a

        total = len(a) + len(b)
        half = total // 2

        left = 0
        right = len(a)
        # This should be left <= right, instead of left < right because
        #  we need to run this loop for left==right at last.
        while left <= right:
            a_mid = (left + right) // 2

            # [x, x, x, ..]
            #        a_mid

            # A_left = A[:a_mid]
            # A_right = A[a_mid:]

            # [x, x, x, ..]
            #        b_mid

            # B_left = A[:b_mid]
            # B_right = A[b_mid:]

            # half = a_mid + b_mid

            b_mid = half - a_mid
            # print(f"{a_mid=}, {a[:a_mid]=}, {b_mid=}, {b[:b_mid]=}")

            a_left = a[a_mid - 1] if a_mid - 1 >= 0 else -inf
            a_right = a[a_mid] if a_mid < len(a) else inf

            b_left = b[b_mid - 1] if b_mid - 1 >= 0 else -inf
            b_right = b[b_mid] if b_mid < len(b) else inf

            if a_left <= b_right and b_left <= a_right:
                # print(f"{a[:a_mid]=}, {b[:b_mid]=}, {total=}, {half=}")
                # odd
                if total % 2:
                    return min(a_right, b_right)
                # even
                return (max(a_left, b_left) + min(a_right, b_right)) / 2
            elif a_left > b_right:
                right = a_mid
            else:
                left = a_mid + 1

        # Unreachable here


# @leetup=code
