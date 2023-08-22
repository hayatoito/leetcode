# @leetup=info id=1143 lang=python3 slug=longest-common-subsequence



# @leetup=code
class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        # N1 = len(text1)
        # N2 = len(text2)
        # dp = [[0] * (N2 + 1) for _ in range(N1 + 1)]
        # for i in range(1, N1 + 1):
        #     for j in range(1, N2 + 1):
        #         dp[i][j] = max(
        #             dp[i - 1][j],
        #             dp[i][j - 1],
        #             dp[i - 1][j - 1] + (1 if text1[i - 1] == text2[j - 1] else 0),
        #         )

        dp = [[0] * (len(text2) + 1) for _ in range(len(text1) + 1)]
        for i, a in enumerate(text1):
            for j, b in enumerate(text2):
                if a == b:
                    dp[i + 1][j + 1] = dp[i][j] + 1
                else:
                    dp[i + 1][j + 1] = max(dp[i + 1][j], dp[i][j + 1])

        return dp[-1][-1]


# @leetup=code

