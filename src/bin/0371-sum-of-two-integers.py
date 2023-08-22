# @leetup=info id=371 lang=python3 slug=sum-of-two-integers




# @leetup=code
class Solution:
    def getSum(self, a: int, b: int) -> int:
        mask = (1 << 32) - 1
        while b & mask:
            (a, b) = (a ^ b, (a & b) << 1)
        return (a & mask) if b else a


# @leetup=code

solution = Solution()
print(solution.getSum(1, 2))
print(solution.getSum(-1, 1))
