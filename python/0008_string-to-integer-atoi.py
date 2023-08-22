class Solution:
    def myAtoi(self, s: str) -> int:
        N = len(s)
        i = 0
        while i < N and s[i] == ' ':
            i += 1

        sign = 1
        if i < N and s[i] == '+':
            i += 1
        elif i < N and s[i] == '-':
            i += 1
            sign = -1

        n = 0
        while i < N and s[i].isdigit():
            d = ord(s[i]) - ord('0')
            n = n * 10 + d
            i += 1

        n = n * sign

        return min(max(n, -pow(2, 31)), pow(2, 31) - 1)
