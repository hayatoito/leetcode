# @leetup=info id=17 lang=python3 slug=letter-combinations-of-a-phone-number



# @leetup=code


class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []

        m = {
            "2": "abc",
            "3": "def",
            "4": "ghi",
            "5": "jkl",
            "6": "mno",
            "7": "pqrs",
            "8": "tuv",
            "9": "wxyz",
        }

        res = []
        letters = []

        def dfs(i: int):
            if i == len(digits):
                res.append("".join(letters))
                return
            for c in m[digits[i]]:
                letters.append(c)
                dfs(i + 1)
                letters.pop()

        dfs(0)

        return res


# @leetup=code
