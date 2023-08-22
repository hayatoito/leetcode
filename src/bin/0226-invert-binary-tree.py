# @leetup=info id=226 lang=python3 slug=invert-binary-tree



from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @leetup=code


class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root:
            return None
        return TreeNode(
            root.val, self.invertTree(root.right), self.invertTree(root.left)
        )


# @leetup=code
