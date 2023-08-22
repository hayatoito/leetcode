# @leetup=custom
# @leetup=info id=23 lang=python3 slug=merge-k-sorted-lists



# @leetup=custom

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


from typing import Optional

# @leetup=code
import heapq

ListNode.__eq__ = lambda self, other: self.val == other.val
ListNode.__lt__ = lambda self, other: self.val < other.val


class Solution:
    def mergeKLists(self, lists: list[Optional[ListNode]]) -> Optional[ListNode]:

        dummy_head = ListNode()
        head = dummy_head
        lists = [a for a in lists if a]

        q = []
        for a in lists:
            heapq.heappush(q, a)

        while q:
            node = heapq.heappop(q)
            head.next = node
            head = head.next
            if node.next:
                heapq.heappush(q, node.next)

        return dummy_head.next


# @leetup=code
