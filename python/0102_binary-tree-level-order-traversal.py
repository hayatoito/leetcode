# Definition for a binary tree node.
class TreeNode(object):
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

from collections import deque

class Solution(object):
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
