# Definition for a binary tree node.
class TreeNode(object):
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution(object):
    def hasPathSum(self, root: TreeNode, targetSum: int) -> bool:

        def check(node: TreeNode, target: int) -> bool:
            if not node:
                return False
            n = target - node.val
            if not node.left and not node.right and n == 0:
                return True
            return check(node.left, n) or check(node.right, n)

        return check(root, targetSum)
