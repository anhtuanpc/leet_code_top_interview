/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution
{
public:
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2)
    {
        ListNode *initialNode = new ListNode();
        ListNode *current = initialNode;
        int carry = 0;

        while (l1 != nullptr || l2 != nullptr)
        {
            int sum = 0;
            int value1 = l1 != nullptr ? l1->val : 0;
            int value2 = l2 != nullptr ? l2->val : 0;
            sum = carry + value1 + value2;
            int digit = sum % 10;
            carry = sum / 10;
            current->next = new ListNode(digit);
            current = current->next;
            l1 = l1 != nullptr ? l1->next : nullptr;
            l2 = l2 != nullptr ? l2->next : nullptr;
        }

        if (carry != 0)
        {
            current->next = new ListNode(carry);
        }

        return initialNode->next;
    }
};