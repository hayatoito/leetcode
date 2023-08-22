# @leetup=info id=973 lang=python3 slug=k-closest-points-to-origin



# @leetup=code

import heapq


class Solution:
    def kClosest(self, points: list[list[int]], k: int) -> list[list[int]]:
        q = []
        for p in points:
            # heapq.heappush(q, (-(p[0] * p[0] + p[1] * p[1]), p))
            # if len(q) > k:
            #     heapq.heappop(q)

            # Use heappushpop
            if len(q) == k:
                heapq.heappushpop(q, (-(p[0] * p[0] + p[1] * p[1]), p))
            else:
                heapq.heappush(q, (-(p[0] * p[0] + p[1] * p[1]), p))

        return [p for (_, p) in q]


# @leetup=code
