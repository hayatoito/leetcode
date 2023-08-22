# @leetup=info id=199 lang=python3 slug=binary-tree-right-side-view



from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @leetup=code


class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> list[int]:
        res = []

        def dfs(n, level):
            if not n:
                return
            if len(res) == level:
                res.append(n.val)
            dfs(n.right, level + 1)
            dfs(n.left, level + 1)

        dfs(root, 0)
        return res


# @leetup=code
