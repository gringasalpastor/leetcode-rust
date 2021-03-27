use listnode::*;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None;
        let mut current = head;

        while let Some(ref mut cur) = current {
            let next = cur.next.take();
            cur.next = previous;

            previous = current;
            current = next;
        }

        return previous;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let f_s = ListNode::from_slice;

        assert_eq!(Solution::reverse_list(f_s(&[1, 2, 3])).wrap().to_string(), f_s(&[3, 2, 1]).wrap().to_string());
    }
}
