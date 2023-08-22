# @leetup=info id=417 lang=python3 slug=pacific-atlantic-water-flow



# @leetup=code

from collections import deque


class Solution:
    def pacificAtlantic(self, heights: list[list[int]]) -> list[list[int]]:
        def bfs(q: deque) -> set:
            visited = set()
            while q:
                (r, c) = q.popleft()
                if (r, c) in visited:
                    continue
                visited.add((r, c))

                for dr, dc in ((0, 1), (0, -1), (-1, 0), (1, 0)):
                    nr = r + dr
                    nc = c + dc
                    if 0 <= nr < R and 0 <= nc < C and heights[r][c] <= heights[nr][nc]:
                        q.append((nr, nc))
            return visited

        R = len(heights)
        C = len(heights[0])

        q = deque()
        for r in range(R):
            q.append((r, 0))
        for c in range(1, C):
            q.append((0, c))

        visited1 = bfs(q)

        q = deque()
        for r in range(R):
            q.append((r, C - 1))
        for c in range(C - 1):
            q.append((R - 1, c))

        visited2 = bfs(q)

        return [list(a) for a in visited1.intersection(visited2)]


# @leetup=code
