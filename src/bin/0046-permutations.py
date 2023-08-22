# @leetup=info id=46 lang=python3 slug=permutations



# @leetup=code


class Solution:
    def permute(self, nums: list[int]) -> list[list[int]]:
        res = []
        perm = []
        picked = set()

        def gen():
            if len(perm) == len(nums):
                res.append(perm[:])
            for n in nums:
                if n not in picked:
                    perm.append(n)
                    picked.add(n)
                    gen()
                    perm.pop()
                    picked.remove(n)

        gen()
        return res


# @leetup=code
