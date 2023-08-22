# @leetup=info id=238 lang=python3 slug=product-of-array-except-self



# @leetup=code
class Solution:
    def productExceptSelf(self, nums: list[int]) -> list[int]:
        N = len(nums)
        prefix_product = [1]
        product = 1
        for n in nums:
            product *= n
            prefix_product.append(product)

        postfix_product = [1]
        product = 1
        for n in reversed(nums):
            product *= n
            postfix_product.append(product)

        return [prefix_product[i] * postfix_product[N - 1 - i] for i in range(N)]


# @leetup=code
