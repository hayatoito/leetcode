# @leetup=info id=20 lang=python3 slug=valid-parentheses



from typing import Optional

# @leetup=code
class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        pairs = {
            ")": "(",
            "}": "{",
            "]": "[",
        }
        for a in s:
            if a in pairs:
                if not stack or pairs[a] != stack.pop():
                    return False
            else:
                stack.append(a)
        return len(stack) == 0


# @leetup=code
