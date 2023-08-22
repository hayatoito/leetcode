# @leetup=info id=21 lang=python3 slug=merge-two-sorted-lists



from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# @leetup=code


class Solution:
    def mergeTwoLists(
        self, list1: Optional[ListNode], list2: Optional[ListNode]
    ) -> Optional[ListNode]:
        a = list1
        b = list2
        dummy_head = ListNode()
        head = dummy_head
        while a or b:
            if not a:
                pick = b
            elif not b:
                pick = a
            else:
                if a.val < b.val:
                    pick = a
                else:
                    pick = b

            head.next = pick
            head = head.next
            if pick == a:
                a = a.next
            else:
                b = b.next

        return dummy_head.next


# @leetup=code
