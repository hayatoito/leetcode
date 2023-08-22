# @leetup=info id=90 lang=python3 slug=subsets-ii



# @leetup=code


class Solution:
    def subsetsWithDup(self, nums: list[int]) -> list[list[int]]:
        nums.sort()
        res = set()
        subsets = []

        def dfs(i: int):
            if i == len(nums):
                res.add(tuple(subsets))
                return

            # pick i-th
            subsets.append(nums[i])
            dfs(i + 1)
            subsets.pop()

            # skip
            dfs(i + 1)

        dfs(0)
        return list(res)


# @leetup=code
