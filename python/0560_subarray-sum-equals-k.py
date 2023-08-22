from collections import defaultdict

class Solution(object):
    def subarraySum(self, nums: list[int], k: int) -> int:
        seen = defaultdict(int)
        total = 0
        count = 0
        # seen: how many subarrays, [0:n], whose sums are key.
        seen[0] = 1
        for n in nums:
            total += n
            # [ ..............a....b]
            # <- prev_total ->
            # <-      total      -->
            # if total - prev_total == k, the sum of subarray [a:b] is k
            prev_total = total - k
            if prev_total in seen:
                count += seen[prev_total]

            seen[total] += 1

        return count
