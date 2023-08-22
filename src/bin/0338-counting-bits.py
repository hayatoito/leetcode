# @leetup=info id=338 lang=python3 slug=counting-bits



# @leetup=code
class Solution:
    def countBits(self, n: int) -> list[int]:
        res = [0] * (n + 1)
        for i in range(n + 1):
            res[i] = res[i // 2] + (i % 2)
        return res


# @leetup=code
