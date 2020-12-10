mod node;

use node::*;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let _l1 = reverse(l1);
        let l2 = reverse(l2);

        // let mut l1_l2_sum = l1.as_ref().unwrap().val + l2.as_ref().unwrap().val;
        // let mut digit = l1_l2_sum % 10;
        // let carry = if l1_l2_sum >= 10 { 1 } else { 0 };
        // let mut sum_head = Some(Box::new(ListNode::new(digit)));

        // let zero_closure = || Box::new(ListNode::new(0));

        // while l1.is_some() || l2.is_some() {
        //     l1_l2_sum = match l1 {
        //         Some(ref x) => x.val,
        //         None => 0,
        //     } + match l2 {
        //         Some(ref x) => x.val,
        //         None => 0,
        //     };
        //     // digit = l1_l2_sum % 10;
        //     // carry = if l1_l2_sum >= 10 {1} else {0};

        //     // sum = Some(Box::new(ListNode::new(digit)));
        // }

        return l2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut example1 = ListNode::from_slice(&[1, 2, 3]);
        let example2 = ListNode::from_slice(&[1, 2, 3]);

        assert_eq!(example1.wrap().to_string(), "1, 2, 3");
        assert_eq!(
            ListNode::from_slice(&[1, 2, 3]).wrap().to_string(),
            "1, 2, 3"
        );
        assert_eq!(ListNode::from_slice(&[]).wrap().to_string(), "");
        example1 = reverse(example1);
        // assert_eq!(example1.as_ref().unwrap().to_string(), "3, 2, 1");

        Solution::add_two_numbers(example1, example2);
    }
}
