# https://www.youtube.com/watch?v=XEmy13g1Qxc

# quick select

# Avg: O(N)
# worst case is O(N^2)

# If you shuffle the array, Can be O(N) for the maliciaous test case


class Solution:
    def findKthLargest(self, nums: list[int], k: int) -> int:
        # Find k-th (zero-based) smallest
        k = len(nums) - k

        def quick_select(l, r):
            pivot = nums[r]
            p = l
            # invariant:
            # nums[l:p] <= pivot
            # nums[p:i] > pivot
            for i in range(l, r):
                if nums[i] <= pivot:
                    nums[p], nums[i] = nums[i], nums[p]
                    p += 1
            nums[p], nums[r] = pivot, nums[p]

            if p > k:
                return quick_select(l, p - 1)
            elif p < k:
                return quick_select(p + 1, r)
            else:
                return nums[p]

        return quick_select(0, len(nums) - 1)
