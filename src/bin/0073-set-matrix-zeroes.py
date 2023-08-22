# @leetup=info id=73 lang=python3 slug=set-matrix-zeroes



# @leetup=code


class Solution:
    def setZeroes(self, matrix: list[list[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """
        R = len(matrix)
        C = len(matrix[0])

        zero_r = set()
        zero_c = set()

        for r in range(R):
            for c in range(C):
                if matrix[r][c] == 0:
                    zero_r.add(r)
                    zero_c.add(c)

        for r in range(R):
            for c in range(C):
                if r in zero_r or c in zero_c:
                    matrix[r][c] = 0


# @leetup=code
