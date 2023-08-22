# @leetup=info id=49 lang=python3 slug=group-anagrams



# @leetup=code
from collections import defaultdict


class Solution:
    def groupAnagrams(self, strs: list[str]) -> list[list[str]]:
        groups = defaultdict(list)
        for s in strs:
            groups[str(sorted(s))].append(s)
        return list(groups.values())


# @leetup=code
