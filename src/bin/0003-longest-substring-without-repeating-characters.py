# @leetup=info id=3 lang=python3 slug=longest-substring-without-repeating-characters

# @leetup=code


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        prev = {}
        start = 0
        result = 0
        for i, c in enumerate(s):
            if c in prev:
                start = max(start, prev[c] + 1)
            prev[c] = i
            result = max(result, i + 1 - start)
        return result


# @leetup=code
