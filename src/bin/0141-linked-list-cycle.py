# @leetup=info id=141 lang=python3 slug=linked-list-cycle



class ListNode(object):
    def __init__(self, x):
        self.val = x
        self.next = None


# @leetup=code


class Solution:
    def hasCycle(self, head: ListNode) -> bool:
        slow = head
        fast = head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
            if fast is slow:
                return True

        return False


# @leetup=code
