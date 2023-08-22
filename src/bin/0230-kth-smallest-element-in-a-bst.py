# @leetup=info id=230 lang=python3 slug=kth-smallest-element-in-a-bst



from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @leetup=code
class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:

        inorder = []

        def dfs(root: Optional[TreeNode]):
            if not root:
                return
            dfs(root.left)
            inorder.append(root.val)
            dfs(root.right)

        dfs(root)
        return inorder[k - 1]


# @leetup=code
