class Solution:

    def isSubsequence(self, s: str, t: str) -> bool:
        if not s:
            return True
        left = 0

        for c in t:
            if s[left] == c:
                left += 1
            if left == len(s):
                return True

        return False
