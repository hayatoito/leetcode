# @leetup=info id=19 lang=python3 slug=remove-nth-node-from-end-of-list


from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# @leetup=code
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy_head = ListNode(0, head)
        a = dummy_head
        b = dummy_head
        for _ in range(n):
            a = a.next
        while True:
            if not a.next:
                b.next = b.next.next
                return dummy_head.next
            a = a.next
            b = b.next


# @leetup=code
