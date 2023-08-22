# @leetup=info id=572 lang=python3 slug=subtree-of-another-tree



from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @leetup=code

class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        def is_identical(a: Optional[TreeNode], b: Optional[TreeNode]) -> bool:
            if (not a) and (not b):
                return True
            if (not a) or (not b):
                return False
            return (
                a
                and b
                and a.val == b.val
                and is_identical(a.left, b.left)
                and is_identical(a.right, b.right)
            )

        def is_subtree(root: Optional[TreeNode], subtree: Optional[TreeNode]) -> bool:
            if not subtree:
                return True
            if not root:
                return False
            if is_identical(root, subtree):
                return True
            return is_subtree(root.left, subtree) or is_subtree(root.right, subtree)

        return is_subtree(root, subRoot)


# @leetup=code
