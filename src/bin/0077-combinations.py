# @leetup=info id=77 lang=python3 slug=combinations



# @leetup=code


class Solution:
    def combine(self, n: int, k: int) -> list[list[int]]:
        res = []
        combi = []

        def dfs(picked: int, remain: int):
            if remain == 0:
                res.append(combi[:])
                return
            if picked == n:
                return

            # pick
            combi.append(picked + 1)
            dfs(picked + 1, remain - 1)
            combi.pop()

            # skip
            dfs(picked + 1, remain)

        dfs(0, k)
        return res


# @leetup=code
