# Ref: https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/solutions/1322101/c-java-python-maxheap-minheap-binary-search-picture-explain-clean-concise/?orderBy=most_votes

import heapq


class Solution:
    def kthSmallest(self, matrix: list[list[int]], k: int) -> int:
        #  1  5  9
        # 10 11 13
        # 12 13 15

        N = len(matrix)
        q = []

        heapq.heappush(q, (matrix[0][0], 0, 0))
        for _ in range(k):
            (n, r, c) = heapq.heappop(q)
            if c + 1 < N:
                heapq.heappush(q, (matrix[r][c + 1], r, c + 1))
            if c == 0 and r + 1 < N:
                heapq.heappush(q, (matrix[r + 1][c], r + 1, c))
        return n
