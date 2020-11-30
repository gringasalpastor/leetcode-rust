use std::fmt;

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

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = write!(f, "{}, ", self.val);
        let mut next = &self.next;
        while next.is_some() {
            let v = next.as_ref().unwrap().val;
            let x = match &next.as_ref().unwrap().next {
                Some(_) => format!("{}, ", v),
                None => format!("{}", v),
            };
            res = write!(f, "{}", x);
            next = &next.as_ref().unwrap().next;
        }
        return res;
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
        match *self.0 {
            Some(ref s) => write!(formatter, "{}", s),
            None => write!(formatter, ""),
        }
    }
}
