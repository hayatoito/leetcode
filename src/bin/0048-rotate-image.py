# @leetup=info id=48 lang=python3 slug=rotate-image



# @leetup=code


class Solution:
    def rotate(self, matrix: list[list[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """

        N = len(matrix)
        left, right = 0, N - 1

        while left < right:
            for i in range(right - left):
                top, bottom = left, right

                # Save the top_left
                top_left = matrix[top][left + i]

                # Move bottom_left into top_left
                matrix[top][left + i] = matrix[bottom - i][left]

                # Move bottom_right into bottom_left
                matrix[bottom - i][left] = matrix[bottom][right - i]

                # Move top_right into bottom_right
                matrix[bottom][right - i] = matrix[top + i][right]

                # Move top_left into top_right
                matrix[top + i][right] = top_left

            left += 1
            right -= 1


# @leetup=code
