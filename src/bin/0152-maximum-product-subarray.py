# @leetup=info id=152 lang=python3 slug=maximum-product-subarray



# @leetup=code
class Solution:
    def maxProduct(self, nums: list[int]) -> int:
        positive_max = 1
        negative_max = 1

        res = -(1 << 32)
        for n in nums:
            if n == 0:
                positive_max = 1
                negative_max = 1
                res = max(res, 0)
            else:
                positive_max, negative_max = (
                    max(positive_max * n, negative_max * n, n),
                    min(positive_max * n, negative_max * n, n),
                )
                res = max(res, positive_max)
        return res


# @leetup=code
