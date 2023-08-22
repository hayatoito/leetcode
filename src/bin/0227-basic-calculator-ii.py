# @leetup=info id=227 lang=python3 slug=basic-calculator-ii



from typing import Optional

# @leetup=code
class Solution:
    def calculate(self, s: str) -> int:
        # Append "+" as sentinel.
        s = s + "+"
        nums = []
        num = 0
        op = "+"
        for c in s:
            if c.isdigit():
                num = num * 10 + ord(c) - ord("0")
            elif c != " ":
                if op == "+":
                    nums.append(num)
                elif op == "-":
                    # 22 - 3 * 5
                    #        ^
                    # nums = [22]
                    # op = '-'
                    # num = 3
                    nums.append(-num)
                    # nums = [22, -3]
                    # op = '*' (later)
                elif op == "*":
                    # 22 - 3 * 5 + 1
                    #            ^
                    # nums = [22, 3]
                    # op = '*'
                    # num = 5

                    nums[-1] *= num
                    # nums = [22, 15]
                    # op = '+'
                    # num = 0
                elif op == "/":
                    nums[-1] = int(float(nums[-1]) / num)

                op = c
                num = 0
        return sum(nums)


# @leetup=code
