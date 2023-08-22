# @leetup=info id=124 lang=python3 slug=binary-tree-maximum-path-sum



from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @leetup=code
import math


class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:

        res = -math.inf

        def max_path(node) -> int:
            nonlocal res
            if not node:
                return 0
            left = max_path(node.left)
            right = max_path(node.right)
            res = max(res, node.val + left + right)
            return max(0, node.val + max(left, right))

        max_path(root)
        return res


# @leetup=code
