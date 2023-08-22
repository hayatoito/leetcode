class Solution:
    def nextPermutation(self, nums: list[int]):
        # [5, 2, 4, 3, 1]

        #      4
        #     / \
        #  5 2   3
        #         \
        #          1

        N = len(nums)
        # Find i such that a[i] < a[i + 1]  (from right to left)
        i = N - 2
        while i >= 0 and nums[i] >= nums[i + 1]:
            i -= 1

        # [5, 2, 4, 3, 1]
        #     ^
        #     i
        if i >= 0:
            # Find j such that nums[i] < nums[j] (from right to left)
            j = N - 1
            while nums[i] >= nums[j]:
                j -= 1
            # [5, 2, 4, 3, 1]
            #     ^     ^
            #     i     j

            # swap nums[i] and nums[j]
            nums[i], nums[j] = nums[j], nums[i]

            # [5, 3, 4, 2, 1]
            #     ^     ^
            #     i     j

            #     4
            #    / \
            # 5 3   2
            #        \
            #         1

        # reverse: nums[i+1:]
        nums[i+1:] = reversed(nums[i+1:])
        # [5, 3, 1, 2, 4]

        #         4
        #        /
        # 5 3   2
        #    \ /
        #     1
