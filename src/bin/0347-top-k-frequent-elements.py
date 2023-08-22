# @leetup=info id=347 lang=python3 slug=top-k-frequent-elements



# @leetup=code

from collections import Counter
import heapq


class Solution(object):
    def topKFrequent(self, nums: list[int], k: int) -> list[int]:
        count = Counter(nums)
        return heapq.nlargest(k, count.keys(), key=count.get)

        # Alternative of heapq.nlargest.
        # heap = []
        # heapq.heapify(heap)

        # for n in count:
        #     heapq.heappush(heap, (-count[n], n))
        # result = []
        # for _ in range(k):
        #     (_, n) = heapq.heappop(heap)
        #     result.append(n)
        # return result


# @leetup=code
