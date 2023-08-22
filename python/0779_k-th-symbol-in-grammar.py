class Solution:
    def kthGrammar(self, n: int, k: int) -> int:
        def kth(n: int, k: int) -> int:
            if n == 0:
                return 0
            pre = kth(n - 1, k // 2)
            if pre == 0:
                if k % 2 == 0:
                    return 0
                else:
                    return 1
            else:
                if k % 2 == 0:
                    return 1
                else:
                    return 0


        return kth(n - 1, k - 1)
