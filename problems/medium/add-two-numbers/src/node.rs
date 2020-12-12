use std::fmt;

// Start provided code
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
// End provided code

impl ListNode {
    #[inline]
    pub fn from_slice(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for &val in values {
            current.replace(Box::new(ListNode::new(val)));
            current = &mut current
                .as_mut()
                .expect("current always assigned on previous line")
                .next;
        }
        return head;
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
