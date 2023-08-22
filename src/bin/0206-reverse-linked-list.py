# @leetup=info id=206 lang=python3 slug=reverse-linked-list




class ListNode(object):
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# @leetup=code


class Solution:
    def reverseList(self, head: ListNode) -> ListNode:
        prev = None
        current = head
        while current:
            next_node = current.next
            current.next = prev
            prev = current
            current = next_node
        return prev


# @leetup=code
