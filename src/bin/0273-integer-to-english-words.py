# @leetup=info id=273 lang=python3 slug=integer-to-english-words



# @leetup=code
class Solution:
    def numberToWords(self, num: int) -> str:
        ones = [
            "",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ]
        assert len(ones) == 20

        tens = [
            "",
            "",
            "Twenty",
            "Thirty",
            "Forty",
            "Fifty",
            "Sixty",
            "Seventy",
            "Eighty",
            "Ninety",
        ]
        assert len(tens) == 10

        def helper(n: int) -> list[str]:
            if n >= 1_000_000_000:
                return (
                    helper(n // 1_000_000_000) + ["Billion"] + helper(n % 1_000_000_000)
                )
            if n >= 1_000_000:
                return helper(n // 1_000_000) + ["Million"] + helper(n % 1_000_000)
            if n >= 1_000:
                return helper(n // 1_000) + ["Thousand"] + helper(n % 1_000)
            if n >= 100:
                return helper(n // 100) + ["Hundred"] + helper(n % 100)
            if n >= 20:
                return [tens[n // 10]] + helper(n % 10)
            return [ones[n]]

        if num == 0:
            return "Zero"
        return " ".join(s for s in helper(num) if s)


# @leetup=code

solution = Solution()
print(solution.numberToWords(10))
print(solution.numberToWords(20))
print(solution.numberToWords(10_200))
print(solution.numberToWords(1_234_567))
