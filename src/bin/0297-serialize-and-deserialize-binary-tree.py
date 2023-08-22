# @leetup=info id=297 lang=python3 slug=serialize-and-deserialize-binary-tree



from typing import Optional


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


# @leetup=code
class Codec:
    NULL = "null"

    def serialize(self, root: TreeNode) -> str:
        """Encodes a tree to a single string.

        :type root: TreeNode
        :rtype: str
        """

        def dfs(n: TreeNode):
            if not n:
                vals.append(Codec.NULL)
            else:
                vals.append(str(n.val))
                dfs(n.left)
                dfs(n.right)

        vals = []
        dfs(root)
        return " ".join(vals)

    def deserialize(self, data: str) -> TreeNode:
        """Decodes your encoded data to tree.

        :type data: str
        :rtype: TreeNode
        """

        def doit() -> TreeNode:
            val = next(vals)
            if val == Codec.NULL:
                return None
            n = TreeNode(int(val))
            n.left = doit()
            n.right = doit()
            return n

        vals = iter(data.split())
        return doit()


# @leetup=code
