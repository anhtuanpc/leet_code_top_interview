// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0); // Dummy node to hold the result
        let (mut p, mut q, mut current) = (l1, l2, &mut dummy);
        let mut carry = 0;

        while p.is_some() || q.is_some() {
            let sum = carry +
                p.as_ref().map_or(0, |node| node.val) +
                q.as_ref().map_or(0, |node| node.val);

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            p = p.and_then(|node| node.next);
            q = q.and_then(|node| node.next);
        }

        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy.next
    }
}