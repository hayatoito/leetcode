# @leetup=info id=98 lang=python3 slug=validate-binary-search-tree



from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @leetup=code
import math


class Solution:
    def isValidBST(self, root: TreeNode) -> bool:
        def is_valid(node: TreeNode, low: int, high: int) -> bool:
            if not node:
                return True
            if not (low < node.val < high):
                return False
            return is_valid(node.left, low, node.val) and is_valid(
                node.right, node.val, high
            )

        return is_valid(root, -math.inf, math.inf)


# @leetup=code
