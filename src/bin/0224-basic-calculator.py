# @leetup=info id=224 lang=python3 slug=basic-calculator




from typing import Optional

# @leetup=code
class Solution:
    def calculate(self, s: str) -> int:
        res = 0
        res_stack = []
        sign_stack = []
        sign = 1
        num = 0
        for c in s:
            if c.isdigit():
                num = num * 10 + (ord(c) - ord("0"))
            elif c == "+":
                # ..res.. sign num +
                res += sign * num
                num = 0
                sign = 1
            elif c == "-":
                # ..res.. sign num -
                res += sign * num
                num = 0
                sign = -1
            elif c == "(":
                # res -[sign] (
                res_stack.append(res)
                sign_stack.append(sign)

                res = 0
                sign = 1
                assert num == 0
            elif c == ")":
                # 3 - (   2     -    4)
                # 3 - ( ..res.. sign num )

                # num_stack = [3]
                # sign_stack = [-1]
                # res = 2
                # sign = -1
                # num = 4

                res += sign * num
                # => res = res + sing * num = 2 + (-1 * 4) = -2

                # 3 - ( -2 )
                sign = sign_stack.pop()
                res *= sign
                # res = res * sign = -2 * -1 = 2

                # 3 + 2
                res += res_stack.pop()
                # res = res + num_stack.pop() = 2 + 3 = 5

                num = 0
                sign = 1
        return res + sign * num


# @leetup=code
