class Solution:

    def shipWithinDays(self, weights: list[int], days: int) -> int:

        def days_needed(capacity: int) -> int:
            day = 1
            weight_sum = 0
            for w in weights:
                if weight_sum + w > capacity:
                    day += 1
                    weight_sum = w
                else:
                    weight_sum += w

            return day

        left = max(weights)
        right = sum(weights)

        while left < right:
            mid = (left + right) // 2
            if days_needed(mid) > days:
                left = mid + 1
            else:
                right = mid
        return left
