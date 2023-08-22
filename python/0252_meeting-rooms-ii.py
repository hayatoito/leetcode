# Premium problem

import heapq

class Interval:

    def __init__(self, start, end):
        self.start = start
        self.end = end

# Another solution which doessn't use heap:
# https://www.youtube.com/watch?v=FdzJmTCVyJU&t=15s        

# The following uses heap

class Solution:
    def meeting_rooms(self, intervals: list[Interval]) -> int:
        intervals.sort(key=lambda x: x.start)
        end_times = []
        res = 0
        for i in intervals:
            while end_times and end_times[0] <= i.start:
                heapq.heappop(end_times)
            heapq.heappush(end_times, i.end)
            res = max(res, len(end_times))
        return res

print(Solution().meeting_rooms([Interval(0, 30), Interval(5, 10), Interval(15, 20)]))
