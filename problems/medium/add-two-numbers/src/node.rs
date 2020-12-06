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

