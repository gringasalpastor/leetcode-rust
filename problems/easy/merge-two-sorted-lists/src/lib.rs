use listnode::*;

use std::cmp;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;

        let mut result_head = None;
        let mut current = &mut result_head;
        while let (Some(a), Some(b)) = (&mut l1, &mut l2) {
            let min = cmp::min(a.val, b.val);
            current.replace(Box::new(ListNode::new(min)));
            current = &mut current.as_mut().expect("current always assigned").next;

            match min {
                _ if a.val == min => l1 = a.next.take(),
                _ => l2 = b.next.take(),
            }
        }

        fn drain(l: &mut Option<Box<ListNode>>, mut current: &mut Option<Box<ListNode>>) {
            while let Some(x) = l {
                current.replace(Box::new(ListNode::new(x.val)));
                current = &mut current.as_mut().expect("current always assigned").next;
                *l = x.next.take();
            }
        };

        drain(&mut l1, &mut current);
        drain(&mut l2, &mut current);

        result_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let f = Solution::merge_two_lists;
        let f_s = ListNode::from_slice;

        assert_eq!(
            f(f_s(&[1, 2, 3]), f_s(&[1, 2, 3])).wrap().to_string(),
            "1, 1, 2, 2, 3, 3"
        );
        assert_eq!(f(f_s(&[]), f_s(&[1])).wrap().to_string(), "1");
        assert_eq!(f(f_s(&[1, 2, 4]), f_s(&[3, 5])).wrap().to_string(), "1, 2, 3, 4, 5");
        assert_eq!(f(f_s(&[]), f_s(&[])).wrap().to_string(), "");
    }
}
