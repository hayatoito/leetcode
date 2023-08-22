# Definition for a binary tree node.
class TreeNode(object):
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution(object):
    def sortedArrayToBST(self, nums: list[int]) -> TreeNode:

        def build(start: int, end: int) -> TreeNode:
            if start == end:
                return None
            mid = (start + end) // 2
            return TreeNode(nums[mid],
                            build(start, mid),
                            build(mid + 1, end),
                            )

        return build(0, len(nums))
