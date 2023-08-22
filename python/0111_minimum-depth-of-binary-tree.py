# Definition for a binary tree node.

class TreeNode(object):
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution(object):
    def minDepth(self, root: TreeNode) -> int:

        def depth(node: TreeNode) -> int:
            if not node:
                return 0
            left = depth(node.left) + 1
            right = depth(node.right) + 1
            if not node.left:
                return right
            if not node.right:
                return left
            return min(left, right)

        return depth(root)
