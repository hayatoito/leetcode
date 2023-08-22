# @leetup=info id=57 lang=python3 slug=insert-interval



from typing import Optional

# @leetup=code
class Solution:
    def insert(
        self, intervals: list[list[int]], newInterval: list[int]
    ) -> list[list[int]]:
        def is_overlap(i1, i2):
            return i1[0] <= i2[1] and i2[0] <= i1[1]

        def merge_overlap(i1, i2):
            return [min(i1[0], i2[0]), max(i1[1], i2[1])]

        left = []
        right = []

        for interval in intervals:
            if is_overlap(interval, newInterval):
                newInterval = merge_overlap(newInterval, interval)
            elif interval[1] < newInterval[0]:
                left.append(interval)
            else:
                right.append(interval)

        return left + [newInterval] + right


# @leetup=code
