# @leetup=info id=78 lang=python3 slug=subsets



# @leetup=code


class Solution:
    def subsets(self, nums: list[int]) -> list[list[int]]:
        res = []
        subset = []

        def gen(i: int):
            if i == len(nums):
                res.append(subset[:])
                return
            # Skip i-th element.
            gen(i + 1)
            # Pick i-th element.
            subset.append(nums[i])
            gen(i + 1)
            subset.pop()

        gen(0)
        return res


# @leetup=code
