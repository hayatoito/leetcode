class Solution:
    def searchMatrix(self, matrix: list[list[int]], target: int) -> bool:
        R = len(matrix)
        C = len(matrix[0])

        left = 0
        right = R * C

        while left < right:
            mid = (left + right) // 2
            r, c = mid // C, mid % C
            if matrix[r][c] == target:
                return True
            if matrix[r][c] < target:
                left = mid + 1
            else:
                right = mid

        return False
