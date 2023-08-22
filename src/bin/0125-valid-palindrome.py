# @leetup=info id=125 lang=python3 slug=valid-palindrome



# @leetup=code


class Solution:
    def isPalindrome(self, s: str) -> bool:
        t = [c.lower() for c in s if c.isalnum()]
        i, j = 0, len(t) - 1
        while i < j:
            if t[i] != t[j]:
                return False
            i += 1
            j -= 1
        return True


# @leetup=code
