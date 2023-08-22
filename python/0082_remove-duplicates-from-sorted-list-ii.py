# Definition for singly-linked list.
class ListNode(object):
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution(object):
    def deleteDuplicates(self, head: ListNode) -> ListNode:
        dummy_head = ListNode(0, head)
        head = dummy_head

        while head.next and head.next.next:
            if head.next.val == head.next.next.val:
                # There is a duplicate. Find all nodes with the val.
                copy = head.next
                while copy.next and copy.val == copy.next.val:
                    copy = copy.next
                # Skip duplicated nodes.
                head.next = copy.next
            else:
                head = head.next

        return dummy_head.next
