# @leetup=info id=5 lang=python3 slug=longest-palindromic-substring



# @leetup=code



class Solution:
    def longestPalindrome(self, s: str) -> str:

        res_start = 0
        res_end = 0

        def check(start: int, end: int):
            nonlocal res_start
            nonlocal res_end
            while 0 <= start and end < len(s) and s[start] == s[end]:
                if res_end - res_start < end - start:
                    res_end = end
                    res_start = start
                start -= 1
                end += 1

        for i in range(len(s)):
            check(i, i)
            check(i, i + 1)

        return s[res_start : res_end + 1]

    # DP version
    #
    # def longestPalindrome(self, s: str) -> str:
    #     @cache
    #     def dfs(start: int, end: int) -> bool:
    #         if start + 1 >= end:
    #             return True
    #         return s[start] == s[end - 1] and dfs(start + 1, end - 1)

    #     res_start = 0
    #     res_end = 0

    #     for i in range(len(s)):
    #         for j in range(i + 1, len(s) + 1):
    #             if dfs(i, j) and res_end - res_start < j - i:
    #                 res_start = i
    #                 res_end = j

    #     return s[res_start:res_end]


# @leetup=code
