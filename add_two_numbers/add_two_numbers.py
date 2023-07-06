# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        initial_node = ListNode()
        process_node = initial_node
        carry = 0
        while l1 or l2:
            value_1 = l1.val if l1 != None else 0
            value_2 = l2.val if l2 != None else 0
            total = carry + value_1 + value_2
            carry = total // 10
            digit = total % 10
            process_node.next = ListNode(digit)
            process_node = process_node.next
            l1 = l1.next if l1 != None else None
            l2 = l2.next if l2 != None else None
        if carry != 0:
            process_node.next = ListNode(carry)
        return initial_node.next