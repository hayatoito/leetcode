# @leetup=info id=435 lang=python3 slug=non-overlapping-intervals



from typing import Optional

# @leetup=code
class Solution:
    def eraseOverlapIntervals(self, intervals: list[list[int]]) -> int:
        intervals.sort()
        res = 0
        end = intervals[0][1]
        for interval in intervals[1:]:
            if interval[0] < end:
                res += 1
                end = min(end, interval[1])
            else:
                end = interval[1]

        return res


# @leetup=code
