use listnode::*;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while let Some(ref mut c) = current {
            while let Some(ref mut x) = c.next {
                if x.val != c.val {
                    break;
                }
                c.next = x.next.take();
            }
            current = &mut c.next;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_duplicates() {
        let dd = Solution::delete_duplicates;
        let f_s = ListNode::from_slice;

        assert_eq!(dd(f_s(&[1, 1, 2])).wrap().to_string(), "1, 2");
        assert_eq!(dd(f_s(&[1, 1, 2, 3, 3])).wrap().to_string(), "1, 2, 3");
        assert_eq!(dd(f_s(&[1, 1, 1])).wrap().to_string(), "1");

        assert_eq!(Solution::delete_duplicates(None), None);
    }
}
