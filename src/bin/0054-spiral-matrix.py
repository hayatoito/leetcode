# @leetup=info id=54 lang=python3 slug=spiral-matrix



# @leetup=code


class Solution:
    def spiralOrder(self, matrix: list[list[int]]) -> list[int]:
        R = len(matrix)
        C = len(matrix[0])

        low_r = 0
        high_r = R

        low_c = 0
        high_c = C

        r = 0
        c = 0

        res = []
        d = ((0, 1), (1, 0), (0, -1), (-1, 0))
        direction = 0
        while low_r <= r < high_r and low_c <= c < high_c:
            res.append(matrix[r][c])
            new_r, new_c = r + d[direction][0], c + d[direction][1]
            if low_r <= new_r < high_r and low_c <= new_c < high_c:
                r, c = new_r, new_c
            else:
                if direction == 0:
                    low_r += 1
                elif direction == 1:
                    high_c -= 1
                elif direction == 2:
                    high_r -= 1
                else:
                    low_c += 1
                direction = (direction + 1) % 4
                r, c = r + d[direction][0], c + d[direction][1]
        return res


# @leetup=code
