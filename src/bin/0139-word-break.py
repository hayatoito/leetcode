# @leetup=info id=139 lang=python3 slug=word-break



# @leetup=code
class Solution:
    def wordBreak(self, s: str, wordDict: list[str]) -> bool:
        N = len(s)
        dp = [False] * (N + 1)
        dp[0] = True

        dic = set(wordDict)
        for end in range(1, N + 1):
            for start in range(0, end):
                if not dp[start]:
                    continue
                subword = s[start:end]
                if subword in dic:
                    dp[end] = True
                    break

        return dp[N]


# @leetup=code
