# @leetup=info id=200 lang=python3 slug=number-of-islands



from typing import Optional

# @leetup=code
class Solution:
    def numIslands(self, grid: list[list[str]]) -> int:
        rows = len(grid)
        columns = len(grid[0])

        def dfs(r: int, c: int):
            if r < 0 or r >= rows or c < 0 or c >= columns:
                return
            if grid[r][c] == "0":
                return
            grid[r][c] = "0"
            dfs(r, c + 1)
            dfs(r, c - 1)
            dfs(r + 1, c)
            dfs(r - 1, c)

        count = 0

        for r in range(rows):
            for c in range(columns):
                if grid[r][c] == "1":
                    count += 1
                    dfs(r, c)
        return count


# @leetup=code
