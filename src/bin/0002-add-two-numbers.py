# @leetup=info id=2 lang=python3 slug=add-two-numbers





class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# @leetup=code


class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        dummy_head = ListNode(0)
        prev_node = dummy_head
        carry = 0
        while l1 or l2 or carry:
            digit = 0
            if l1:
                digit += l1.val
            if l2:
                digit += l2.val
            digit += carry
            carry = digit // 10
            digit = digit % 10

            new_node = ListNode(digit)
            prev_node.next = new_node
            prev_node = new_node
            if l1:
                l1 = l1.next
            if l2:
                l2 = l2.next

        return dummy_head.next


# @leetup=code
