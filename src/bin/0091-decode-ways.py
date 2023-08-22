# @leetup=info id=91 lang=python3 slug=decode-ways



# @leetup=code
class Solution:
    def numDecodings(self, s: str) -> int:
        valid = set(str(i) for i in range(1, 27))
        dp = [0] * (len(s) + 1)
        dp[0] = 1
        for i in range(1, len(s) + 1):
            for j in range(i):
                substr = s[j:i]
                if substr in valid:
                    dp[i] += dp[j]
        return dp[-1]


# @leetup=code
