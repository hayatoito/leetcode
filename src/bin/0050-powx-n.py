# @leetup=info id=50 lang=python3 slug=powx-n



# @leetup=code
class Solution:
    def myPow(self, x: float, n: int) -> float:
        def pow(x: float, n: int) -> float:
            if n == 1:
                return x
            if n % 2 == 0:
                return self.myPow(x * x, n // 2)
            return self.myPow(x * x, n // 2) * x

        if n == 0:
            return 1
        if n < 0:
            return 1.0 / pow(x, -n)
        return pow(x, n)


# @leetup=code
