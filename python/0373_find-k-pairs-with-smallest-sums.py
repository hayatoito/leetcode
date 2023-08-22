import heapq

class Solution:
    def kSmallestPairs(self, nums1: list[int], nums2: list[int], k: int) -> list[list[int]]:
        self.nums1 = nums1
        self.nums2 = nums2
        self.visited = set()
        self.heap = []
        heapq.heapify(self.heap)

        self.visit(0, 0)

        result = []

        while self.heap and len(result) < k:
            (_sum, i, j) = heapq.heappop(self.heap)
            result.append((nums1[i], nums2[j]))
            self.visit(i + 1, j)
            self.visit(i, j + 1)
        return result

    def visit(self, i: int, j: int):
        if i == len(self.nums1):
            return
        if j == len(self.nums2):
            return
        if (i, j) in self.visited:
            return
        self.visited.add((i, j))
        sum = self.nums1[i] + self.nums2[j]
        node = (sum, i, j)
        heapq.heappush(self.heap, node)
