from functools import cache

class Solution:
    def maxProfit(self, prices: List[int]) -> int:

        @cache
        def doit(buy: int, day: int) -> int:
            if day == len(prices):
                return 0
            price = prices[day]
            if buy == -1:
                return doit(price, day + 1)

            if buy < price:
                # Sell it
                profit1 = price - buy + doit(-1, day + 1)
                # or don't sell
                profit2 = doit(buy, day + 1)
                return max(profit1, profit2)
            else:
                # buy >= price:
                return doit(price, day + 1)

        return doit(-1, 0)
