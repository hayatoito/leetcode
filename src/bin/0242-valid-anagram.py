# @leetup=info id=242 lang=python3 slug=valid-anagram



# @leetup=code

from collections import Counter


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        return Counter(s) == Counter(t)


# @leetup=code
