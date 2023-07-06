/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
 func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
    initialNode := &ListNode{}
    processNode := initialNode
    carry := 0
    for l1 != nil || l2 != nil || carry != 0 {
        total := carry
        if l1 != nil {
            total += l1.Val
            l1 = l1.Next
        }
        if l2 != nil {
            total += l2.Val
            l2 = l2.Next
        }
        carry = total / 10
        digit := total % 10
        processNode.Next = &ListNode{ Val: digit }
        processNode = processNode.Next
    }

    return initialNode.Next
}