pub mod node;

use node::*;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = &l1;
        let mut l2 = &l2;

        let mut head = None;
        let mut current = &mut head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() {
            let sum = l1.as_ref().map_or(0, |n| n.val) + l2.as_ref().map_or(0, |n| n.val) + carry;
            let digit = sum % 10;
            carry = sum / 10;

            current.replace(Box::new(ListNode::new(digit)));
            current = &mut current
                .as_mut()
                .expect("current always assigned on previous line")
                .next;

            if let Some(l) = l1 {
                l1 = &l.next;
            }
            if let Some(l) = l2 {
                l2 = &l.next;
            }
        }

        return head;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let empty = ListNode::from_slice(&[]);
        let ex123 = ListNode::from_slice(&[1, 2, 3]);
        let ex321 = ListNode::from_slice(&[3, 2, 1]);

        assert_eq!(ex123_rev.wrap().to_string(), "3, 2, 1");
        assert_eq!(ex123.wrap().to_string(), "1, 2, 3");
        assert_eq!(empty.wrap().to_string(), "");
        assert_eq!(
            Solution::add_two_numbers(ex123_rev, ex123)
                .wrap()
                .to_string(),
            "4, 4, 4"
        );

        let ex243 = ListNode::from_slice(&[2, 4, 3]);
        let ex564 = ListNode::from_slice(&[5, 6, 4]);
        assert_eq!(
            Solution::add_two_numbers(ex243, ex564).wrap().to_string(),
            "7, 0, 8"
        );

        let ex9999999 = ListNode::from_slice(&[9, 9, 9, 9, 9, 9, 9]);
        let ex9999 = ListNode::from_slice(&[9, 9, 9, 9]);
        assert_eq!(
            Solution::add_two_numbers(ex9999999, ex9999)
                .wrap()
                .to_string(),
            "8, 9, 9, 9, 0, 0, 0, 1"
        );
    }
}
