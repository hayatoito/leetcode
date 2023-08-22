# @leetup=info id=143 lang=python3 slug=reorder-list


from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# @leetup=code
class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        """
        Do not return anything, modify head in-place instead.
        """

        if not head:
            return

        # Step 1: find middle
        # [1,2,3,4,5]
        #  ^ slow
        #  ^ fast
        slow, fast = head, head
        while fast.next and fast.next.next:
            slow = slow.next
            fast = fast.next.next
        # [1,2,3,4,5]
        #      ^ slow
        #          ^ fast

        # Step 2: reverse second half
        prev, curr = None, slow.next
        # [1,2,3,4,5]
        #        ^ cur
        while curr:
            next_node = curr.next
            curr.next = prev
            prev, curr = curr, next_node
        # [1,2,3,4,5]
        #      ^ slow
        # [4 <- 5]
        #       ^ prev

        slow.next = None
        # [1,2,3]
        #      ^ slow

        # Step 3: merge lists
        head1, head2 = head, prev
        # [1,2,3]
        #  ^ head1
        # [5, 4]
        #  ^ head2
        while head2:
            next_node = head1.next
            # [1,2,3]
            #  ^ head1
            #    ^ next_node
            # [5, 4]
            #  ^ head2
            head1.next = head2
            # [1,5,4]
            #  ^ head1
            #    ^ head2
            #   [2, 3]
            #    ^ next_node
            head1 = head2
            head2 = next_node
            # [1,5,4]
            #    ^ head1
            # [2,3]
            #  ^ head2


# @leetup=code
