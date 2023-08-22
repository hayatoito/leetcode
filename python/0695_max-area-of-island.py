class Solution:

    def maxAreaOfIsland(self, grid: list[list[int]]) -> int:
        rows = len(grid)
        columns = len(grid[0])

        def dfs(r: int, c: int) -> int:
            if not (0 < r < rows and 0 < c < columns):
                return 0
            if grid[r][c] == 0:
                return 0
            grid[r][c] = 0
            return 1 + dfs(r, c + 1) + dfs(r, c - 1) + dfs(r + 1, c) + dfs(r - 1, c)

        result = 0

        for r in range(rows):
            for c in range(columns):
                if grid[r][c] == 1:
                    result = max(result, dfs(r, c))

        return result
