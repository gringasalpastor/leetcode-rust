use listnode::*;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut length = 0;

        let mut current = &head;
        while let Some(ref cur) = current {
            length += 1;
            current = &cur.next;
        }
        let index = length - n - 1;

        let mut current = &mut head;
        let mut pos = 0;
        while let Some(ref mut cur) = current {
            if pos == index {
                cur.next = cur.next.as_mut().unwrap_or(&mut Box::new(ListNode::new(0))).next.take();
                break;
            }
            pos += 1;
            current = &mut cur.next;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_nth_from_end() {
        let rnfe = Solution::remove_nth_from_end;
        let f_s = ListNode::from_slice;
        let ex12345 = f_s(&[1, 2, 3, 4, 5]);
        assert_eq!(rnfe(ex12345, 2).wrap().to_string(), "1, 2, 3, 5");
    }
}
