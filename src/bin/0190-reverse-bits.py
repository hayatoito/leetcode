# @leetup=info id=190 lang=python3 slug=reverse-bits



# @leetup=code


class Solution:
    def reverseBits(self, n: int) -> int:
        a = 0
        for _ in range(32):
            a = (a << 1) | (n & 1)
            n = n >> 1
        return a


# @leetup=code
