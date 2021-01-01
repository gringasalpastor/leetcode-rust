use std::fmt;

// START PROVIDED CODE
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// END PROVIDED CODE

impl ListNode {
    pub fn from_slice(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for &val in values {
            current.replace(Box::new(ListNode::new(val)));
            current = &mut current.as_mut().expect("current always assigned on previous line").next;
        }
        return head;
    }

    pub fn reverse(list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None;
        let mut current = list;

        while let Some(ref mut cur) = current {
            let next = cur.next.take();
            cur.next = previous;

            previous = current;
            current = next;
        }

        return previous;
    }
}

// Wrapper type so we can define `fmt::Display` for `Option<Box<ListNode>>`
pub struct ListNodeWrapper<'a>(&'a Option<Box<ListNode>>);

pub trait Wrap {
    fn wrap<'a>(&'a self) -> ListNodeWrapper<'a>;
}

impl Wrap for Option<Box<ListNode>> {
    fn wrap<'a>(&'a self) -> ListNodeWrapper<'a> {
        ListNodeWrapper(self)
    }
}

impl<'a> fmt::Display for ListNodeWrapper<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut node_opt = self.0;
        while let Some(node) = node_opt {
            let x = match &node.next {
                Some(_) => format!("{}, ", node.val),
                None => format!("{}", node.val),
            };
            write!(formatter, "{}", x)?;
            node_opt = &node.next;
        }
        return Ok(()); // Errors checked above via `?` operator
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
        let ex123_rev = ListNode::reverse(ex123);
        assert_eq!(ex123_rev.wrap().to_string(), "3, 2, 1");
    }
}
