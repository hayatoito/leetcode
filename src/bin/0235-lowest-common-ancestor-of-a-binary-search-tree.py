# @leetup=info id=235 lang=python3 slug=lowest-common-ancestor-of-a-binary-search-tree



from typing import Optional

class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


# @leetup=code
class Solution:
    def lowestCommonAncestor(
        self, root: TreeNode, p: TreeNode, q: TreeNode
    ) -> TreeNode:
        if root.val == p.val or root.val == q.val:
            return root
        if min(p.val, q.val) < root.val < max(p.val, q.val):
            return root
        if p.val < root.val:
            return self.lowestCommonAncestor(root.left, p, q)
        return self.lowestCommonAncestor(root.right, p, q)


# @leetup=code
