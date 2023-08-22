# @leetup=info id=121 lang=python3 slug=best-time-to-buy-and-sell-stock



# @leetup=code


class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        max_profit = 0
        min_price = prices[0]
        for price in prices[1:]:
            max_profit = max(max_profit, price - min_price)
            min_price = min(min_price, price)
        return max_profit


# @leetup=code
