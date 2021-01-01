use listnode::*;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(_l1: Option<Box<ListNode>>, _l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        ListNode::from_slice(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let empty = ListNode::from_slice(&[]);
        let ex123 = ListNode::from_slice(&[1, 2, 3]);
        let ex321 = ListNode::from_slice(&[3, 2, 1]);

        assert_eq!(ex321.wrap().to_string(), "3, 2, 1");
        assert_eq!(ex123.wrap().to_string(), "1, 2, 3");
        assert_eq!(empty.wrap().to_string(), "");
    }
}
