# @leetup=info id=438 lang=python3 slug=find-all-anagrams-in-a-string



# @leetup=code

from collections import Counter


class Solution:
    def findAnagrams(self, s: str, p: str) -> list[int]:
        p_counter = Counter(p)

        s_counter = Counter()
        res = []
        for i, c in enumerate(s):
            s_counter[c] += 1
            start = i - len(p) + 1
            if start - 1 >= 0:
                s_counter[s[start - 1]] -= 1
            if start >= 0 and p_counter == s_counter:
                res.append(start)
        return res


# @leetup=code
