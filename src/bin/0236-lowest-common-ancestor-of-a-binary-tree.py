# @leetup=info id=236 lang=python3 slug=lowest-common-ancestor-of-a-binary-tree



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
        found = None

        def lowest(n: TreeNode) -> bool:
            if not n:
                return None
            nonlocal found
            cnt = 0
            if n.val == p.val or n.val == q.val:
                cnt += 1
            if lowest(n.left):
                cnt += 1
            if lowest(n.right):
                cnt += 1
            if cnt == 2:
                found = n
            return cnt > 0

        lowest(root)

        return found


# @leetup=code
