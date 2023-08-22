class Solution:
    def convert(self, s: str, numRows: int) -> str:
        if numRows == 1:
            return s
        rows = [[] for _ in range(numRows)]
        direction = 1
        r = 0
        for c in s:
            rows[r].append(c)
            if direction == 1:
                if r == numRows - 1:
                    direction = -1
                    r -= 1
                else:
                    r += 1
            else:
                if r == 0:
                    direction = 1
                    r += 1
                else:
                    r -= 1

        return ''.join(''.join(r) for r in rows)
