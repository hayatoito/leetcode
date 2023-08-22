# @leetup=info id=102 lang=python3 slug=binary-tree-level-order-traversal



from typing import Optional


class TreeNode(object):
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @leetup=code
from collections import deque


class Solution:
    def levelOrder(self, root: TreeNode) -> list[list[int]]:
        if not root:
            return []
        current_queue = deque()
        next_queue = deque()
        current_queue.append(root)
        result = []

        while current_queue:
            result.append([n.val for n in current_queue])
            while current_queue:
                node = current_queue.popleft()
                if node.left:
                    next_queue.append(node.left)
                if node.right:
                    next_queue.append(node.right)
            current_queue = next_queue
            next_queue = deque()

        return result


# @leetup=code
