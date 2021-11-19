use listnode::*;
use std::mem;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while current.is_some() && current.as_ref().unwrap().next.is_some() {
            let mut tmp_b = current.as_mut().unwrap().next.take();
            let tmp_c = tmp_b.as_mut().unwrap().next.take();

            mem::swap(current, &mut tmp_b);
            tmp_b.as_mut().unwrap().next = tmp_c;
            current.as_mut().unwrap().next = tmp_b;
            current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn swap_pairs() {
        let swap_p = Solution::swap_pairs;
        let f_s = ListNode::from_slice;

        let ex1234 = f_s(&[1, 2, 3, 4]);
        let ex12345 = f_s(&[1, 2, 3, 4, 5]);
        assert_eq!(swap_p(ex1234).wrap().to_string(), "2, 1, 4, 3");
        assert_eq!(swap_p(ex12345).wrap().to_string(), "2, 1, 4, 3, 5");
        assert_eq!(swap_p(f_s(&[])).wrap().to_string(), "");
        assert_eq!(swap_p(f_s(&[1])).wrap().to_string(), "1");
    }
}
