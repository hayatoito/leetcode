# @leetup=info id=22 lang=python3 slug=generate-parentheses



# @leetup=code


class Solution:
    def generateParenthesis(self, n):

        res = []
        paren = []

        def gen(open_paren: int, close_paren: int):
            if open_paren == n and close_paren == n:
                res.append("".join(paren))
                return

            if open_paren < n:
                paren.append("(")
                gen(open_paren + 1, close_paren)
                paren.pop()

            if open_paren > close_paren:
                paren.append(")")
                gen(open_paren, close_paren + 1)
                paren.pop()

        gen(0, 0)
        return res


# @leetup=code
