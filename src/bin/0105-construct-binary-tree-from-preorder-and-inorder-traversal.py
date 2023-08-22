# @leetup=info id=105 lang=python3 slug=construct-binary-tree-from-preorder-and-inorder-traversal



from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @leetup=code


class Solution:
    def buildTree(self, preorder: list[int], inorder: list[int]) -> TreeNode:

        preorder_index = 0

        def build(inorder: list[int]) -> TreeNode:
            if not inorder:
                return None
            nonlocal preorder_index
            assert preorder_index < len(preorder)
            root = preorder[preorder_index]
            preorder_index += 1
            root_index_in_inorder = inorder.index(root)

            return TreeNode(
                root,
                build(inorder[:root_index_in_inorder]),
                build(inorder[root_index_in_inorder + 1 :]),
            )

        return build(inorder)


# @leetup=code
