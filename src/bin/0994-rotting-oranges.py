# @leetup=info id=994 lang=python3 slug=rotting-oranges



from typing import Optional

# @leetup=code
from collections import deque


class Solution:
    def orangesRotting(self, grid: list[list[int]]) -> int:
        R = len(grid)
        C = len(grid[0])
        q = deque()
        for r in range(R):
            for c in range(C):
                if grid[r][c] == 2:
                    q.append(((r, c), 0))

        res = 0
        visited = set()
        while q:
            ((r, c), dist) = q.popleft()
            if not (0 <= r < R and 0 <= c < C):
                continue
            if (r, c) in visited:
                continue
            visited.add((r, c))
            if grid[r][c] == 0:
                continue
            if grid[r][c] == 1:
                grid[r][c] = 2
                res = dist
            q.append(((r, c + 1), dist + 1))
            q.append(((r, c - 1), dist + 1))
            q.append(((r + 1, c), dist + 1))
            q.append(((r - 1, c), dist + 1))

        for r in range(R):
            for c in range(C):
                if grid[r][c] == 1:
                    return -1

        return res


# @leetup=code
