# @leetup=info id=383 lang=python3 slug=ransom-note



# @leetup=code

from collections import Counter


class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        a = Counter(ransomNote)
        b = Counter(magazine)
        return all(a[k] <= b[k] for k in a)


# @leetup=code
