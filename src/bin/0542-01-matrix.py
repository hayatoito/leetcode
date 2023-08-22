# @leetup=info id=542 lang=python3 slug=01-matrix



from typing import Optional

# @leetup=code
from collections import deque
import math


class Solution:
    def updateMatrix(self, mat: list[list[int]]) -> list[list[int]]:
        R = len(mat)
        C = len(mat[0])

        res = [[math.inf] * C for _ in range(R)]
        q = deque()

        for r in range(R):
            for c in range(C):
                if mat[r][c] == 0:
                    q.append(((r, c), 0))

        visited = set()
        while q:
            ((r, c), dist) = q.popleft()
            if not (0 <= r < R and 0 <= c < C):
                continue
            if (r, c) in visited:
                continue
            visited.add((r, c))
            res[r][c] = min(res[r][c], dist)
            q.append(((r, c + 1), dist + 1))
            q.append(((r, c - 1), dist + 1))
            q.append(((r + 1, c), dist + 1))
            q.append(((r - 1, c), dist + 1))

        return res


# @leetup=code
