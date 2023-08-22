# Definition for a binary tree node.
class TreeNode(object):
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution(object):
    def mergeTrees(self, root1: TreeNode, root2: TreeNode) -> TreeNode:

        def merge(root1: TreeNode, root2: TreeNode) -> TreeNode:
            if not root1:
                return root2
            if not root2:
                return root1
            return TreeNode(root1.val + root2.val,
                            merge(root1.left, root2.left),
                            merge(root1.right, root2.right))

        return merge(root1, root2)
