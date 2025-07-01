// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut curr = result;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() {
            let mut sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = l1.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = l2.next;
            }

            carry = sum / 10;
            cur.val = sum;
            cur.next = Box::new(ListNode::new(sum % 10));
            cur = cur.next;
            sum = sum % 10;
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::add_two_numbers(to_list(vec![2, 4, 3], to_list(vec![5, 6, 4]))),
        to_list(vec![7, 0, 8])
    )
}
