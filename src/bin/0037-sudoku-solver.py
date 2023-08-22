# @leetup=info id=37 lang=python3 slug=sudoku-solver



# @leetup=code


class Solution:
    def solveSudoku(self, board: list[list[str]]) -> None:
        R = len(board)
        C = len(board[0])

        def square(i: int):
            i -= i % 3
            return range(i, i + 3)

        def is_valid(r: int, c: int, n: str) -> bool:
            return (
                all(board[r][i] != n for i in range(C))
                and all(board[i][c] != n for i in range(R))
                and all(board[i][j] != n for i in square(r) for j in square(c))
            )

        def dfs(i: int) -> bool:
            if i == R * C:
                return True
            r = i // C
            c = i % R
            if board[r][c] != ".":
                return dfs(i + 1)
            for n in range(1, 10):
                n = str(n)
                if is_valid(r, c, n):
                    board[r][c] = n
                    if dfs(i + 1):
                        return True
                    board[r][c] = "."
            return False

        assert dfs(0)


# @leetup=code
