use std::cell::RefCell;
use std::rc::Rc;

// START PROVIDED CODE
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val: val, left: None, right: None }
    }
}
// END PROVIDED CODE

pub fn make_node(val: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode {
        val: val,
        left: match left {
            Some(n) => Some(Rc::new(RefCell::new(n))),
            None => None,
        },
        right: match right {
            Some(n) => Some(Rc::new(RefCell::new(n))),
            None => None,
        },
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
