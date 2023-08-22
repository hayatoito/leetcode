class Solution:

    def combinationSum(self, candidates: list[int], target: int) -> list[list[int]]:

        candidates.sort()

        res = []
        combi = []
        N = len(candidates)

        def dfs(index: int, remain: int):
            if index == N:
                if remain == 0:
                    res.append(combi[:])
                return
            if remain < 0:
                return

            # Pick index-th element
            combi.append(candidates[index])
            dfs(index, remain - candidates[index])
            combi.pop()

            # Skip index-th element
            dfs(index + 1, remain)

        dfs(0, target)
        return res
