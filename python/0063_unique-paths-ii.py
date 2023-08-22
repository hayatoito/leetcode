class Solution(object):
    def uniquePathsWithObstacles(self, obstacleGrid: list[list[int]]) -> int:
        R = len(obstacleGrid)
        C = len(obstacleGrid[0])

        dp = [[0] * C for _ in range(R)]

        if obstacleGrid[0][0]:
            return 0

        dp[0][0] = 1
        for r in range(1, R):
            if not obstacleGrid[r][0] and dp[r-1][0]:
                dp[r][0] = 1
        for c in range(1, C):
            if not obstacleGrid[0][c] and dp[0][c-1]:
                dp[0][c] = 1

        for r in range(1, R):
            for c in range(1, C):
                if obstacleGrid[r][c]:
                    continue
                dp[r][c] = dp[r-1][c] + dp[r][c-1]

        return dp[-1][-1]
