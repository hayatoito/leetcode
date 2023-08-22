# @leetup=info id=424 lang=python3 slug=longest-repeating-character-replacement



# @leetup=code

from collections import defaultdict


class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        def feasible(longest: int) -> bool:
            freq = defaultdict(int)
            max_freq = 0
            for i, c in enumerate(s):
                freq[c] += 1
                if i - longest >= 0:
                    freq[s[i - longest]] -= 1
                max_freq = max(max_freq, freq[c])
                if i >= k - 1:
                    if longest - max_freq <= k:
                        return True
            return False

        left = 1
        # +1 is required because len(s) can be feasible.
        right = len(s) + 1
        while left < right:
            mid = (left + right) // 2
            if feasible(mid):
                left = mid + 1
            else:
                right = mid
        return left - 1


# @leetup=code
