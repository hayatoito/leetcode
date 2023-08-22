# @leetup=info id=239 lang=python3 slug=sliding-window-maximum



# @leetup=code

from collections import deque


class Solution:
    def maxSlidingWindow(self, nums: list[int], k: int) -> list[int]:
        N = len(nums)
        # q stores index
        q = deque()
        res = []
        for i in range(N):
            n = nums[i]
            # Make q monotonic decreasing queue.
            while q and nums[q[-1]] <= n:
                q.pop()
            q.append(i)

            if q[0] < i - (k - 1):
                # q[0] is out of sliding window.
                q.popleft()

            if i >= k - 1:
                res.append(nums[q[0]])

        return res


# @leetup=code
