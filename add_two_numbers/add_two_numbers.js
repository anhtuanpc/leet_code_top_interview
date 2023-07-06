/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function (l1, l2) {
  let initialNode = new ListNode(0);
  let processNode = initialNode;
  let carry = 0;

  while (l1 != null || l2 != null) {
    let value1 = l1 != null ? l1.val : 0;
    let value2 =  null ? l2.val : 0;
    let total = carry + value1 + value2;
    let nextValue = total % 10;
    carry = Math.floor(total / 10);

    processNode.next = new ListNode(nextValue);
    processNode = processNode.next;
    l1 = l1 != null ? l1.next : null;
    l2 = l2 != null ? l2.next : null;
  }

  if (carry > 0) {
    processNode.next = new ListNode(carry);
  }

  return initialNode.next;
};
