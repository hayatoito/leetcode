# @leetup=info id=191 lang=python3 slug=number-of-1-bits



# @leetup=code
class Solution:
    def hammingWeight(self, n: int) -> int:
        cnt = 0
        while n:
            n = n & (n - 1)
            cnt += 1
        return cnt


# @leetup=code
