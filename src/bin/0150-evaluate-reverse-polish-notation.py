# @leetup=info id=150 lang=python3 slug=evaluate-reverse-polish-notation



from typing import Optional

# @leetup=code
class Solution:
    def evalRPN(self, tokens: list[str]) -> int:
        s = []
        for a in tokens:
            if a not in "+-*/":
                s.append(int(a))
                continue

            a2 = s.pop()
            a1 = s.pop()
            if a == "+":
                s.append(a1 + a2)
            elif a == "-":
                s.append(a1 - a2)
            elif a == "*":
                s.append(a1 * a2)
            elif a == "/":
                # a3 = abs(a1) // abs(a2)
                # if a1 < 0 < a2 or a2 < 0 < a1:
                #     a3 = -a3
                # s.append(a3)
                s.append(int(float(a1) / a2))
        assert len(s) == 1
        return s[0]


# @leetup=code
