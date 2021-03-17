use listnode::*;

pub struct Solution;

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut cur_node = &mut head;

        loop {
            match cur_node {
                Some(ref mut node) if node.val == val => *cur_node = node.next.take(),
                Some(ref mut node) => cur_node = &mut node.next,
                None => return head,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_elements() {
        let re = Solution::remove_elements;
        let f_s = ListNode::from_slice;

        assert_eq!(re(f_s(&[1, 2, 6, 3, 4, 5, 6]), 6).wrap().to_string(), f_s(&[1, 2, 3, 4, 5]).wrap().to_string());
        assert_eq!(re(f_s(&[7, 7, 7, 7]), 7).wrap().to_string(), f_s(&[]).wrap().to_string());
    }
}
