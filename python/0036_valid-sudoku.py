class Solution:
    def isValidSudoku(self, board: list[list[str]]) -> bool:

        for r in range(9):
            s = set()
            for c in range(9):
                x = board[r][c]
                if x == ".":
                    continue
                if x in s:
                    return False
                s.add(x)

        for c in range(9):
            s = set()
            for r in range(9):
                x = board[r][c]
                if x == ".":
                    continue
                if x in s:
                    return False
                s.add(x)

        for i in range(9):
            sr = (i // 3) * 3
            sc = (i % 3) * 3
            s = set()
            for j in range(9):
                r = sr + j // 3
                c = sc + j % 3
                x = board[r][c]
                if x == ".":
                    continue
                if x in s:
                    return False
                s.add(x)

        return True
