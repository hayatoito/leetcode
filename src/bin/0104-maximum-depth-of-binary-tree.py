# @leetup=info id=104 lang=python3 slug=maximum-depth-of-binary-tree



from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @leetup=code
class Solution(object):
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        def dfs(node: TreeNode) -> int:
            if not node:
                return 0
            return 1 + max(dfs(node.left), dfs(node.right))

        return dfs(root)


# @leetup=code
