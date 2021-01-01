use listnode::*;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
            current = &mut current.as_mut().expect("current always assigned on previous line").next;

            if let Some(l) = l1 {
                l1 = &l.next;
            }
            if let Some(l) = l2 {
                l2 = &l.next;
            }
        }

        if carry == 1 {
            current.replace(Box::new(ListNode::new(1)));
        }

        return head;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let atn = Solution::add_two_numbers;
        let f_s = ListNode::from_slice;

        let ex123 = f_s(&[1, 2, 3]);
        let ex321 = f_s(&[3, 2, 1]);
        assert_eq!(atn(ex321, ex123).wrap().to_string(), "4, 4, 4");

        let ex243 = f_s(&[2, 4, 3]);
        let ex564 = f_s(&[5, 6, 4]);
        assert_eq!(atn(ex243, ex564).wrap().to_string(), "7, 0, 8");

        let ex9999999 = f_s(&[9, 9, 9, 9, 9, 9, 9]);
        let ex9999 = f_s(&[9, 9, 9, 9]);
        assert_eq!(atn(ex9999999, ex9999).wrap().to_string(), "8, 9, 9, 9, 0, 0, 0, 1");
    }
}
