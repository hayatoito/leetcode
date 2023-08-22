# @leetup=info id=295 lang=python3 slug=find-median-from-data-stream



# @leetup=code

import heapq


class MedianFinder:
    def __init__(self):
        # max heap
        self.small = []
        # min heap
        self.large = []

    def addNum(self, num: int) -> None:
        heapq.heappush(self.small, -num)

        if self.small and self.large and -self.small[0] > self.large[0]:
            heapq.heappush(self.large, -heapq.heappop(self.small))
        if len(self.small) > len(self.large) + 1:
            heapq.heappush(self.large, -heapq.heappop(self.small))
        if len(self.small) + 1 < len(self.large):
            heapq.heappush(self.small, -heapq.heappop(self.large))

    def findMedian(self) -> float:
        if len(self.small) == len(self.large):
            return (-self.small[0] + self.large[0]) / 2
        if len(self.small) == len(self.large) + 1:
            return -self.small[0]
        if len(self.small) + 1 == len(self.large):
            return self.large[0]
        assert False


# @leetup=code
