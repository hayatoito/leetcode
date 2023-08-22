# @leetup=info id=76 lang=python3 slug=minimum-window-substring



# @leetup=code

from collections import Counter
from math import inf



class Solution:
    def minWindow(self, s: str, t: str) -> str:
        tc = Counter(t)
        sc = Counter()

        left = 0
        goal = len(tc)

        min_size = inf
        min_left = -1
        for right, c in enumerate(s):
            sc[c] += 1
            if sc[c] == tc[c]:
                goal -= 1
            while left <= right and goal == 0:
                size = right - left + 1
                if min_size > size:
                    min_size = size
                    min_left = left
                left_c = s[left]
                if left_c in tc and sc[left_c] == tc[left_c]:
                    goal += 1
                sc[left_c] -= 1
                left += 1

        return s[min_left : min_left + min_size] if min_left != -1 else ""










# @leetup=code
