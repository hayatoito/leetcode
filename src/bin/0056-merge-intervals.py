# @leetup=info id=56 lang=python3 slug=merge-intervals



from typing import Optional

# @leetup=code
class Solution:
    def merge(self, intervals: list[list[int]]) -> list[list[int]]:
        # def is_overlap(i1, i2):
        #     return i1[0] <= i2[1] and i2[0] <= i1[1]

        # def merge_overlap(i1, i2):
        #     return [min(i1[0], i2[0]), max(i1[1], i2[1])]

        intervals.sort()
        res = []
        for interval in intervals:
            # if res and is_overlap(res[-1], interval):
            if res and interval[0] <= res[-1][1]:
                # res[-1] = merge_overlap(res[-1], interval)
                res[-1][1] = max(res[-1][1], interval[1])
            else:
                res.append(interval)

        return res


# @leetup=code
