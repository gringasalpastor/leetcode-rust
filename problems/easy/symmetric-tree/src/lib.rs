pub struct Solution;

use treenode::*;

use std::cell::RefCell;
use std::rc::Rc;

fn check(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (left, right) {
        (Some(l), Some(r)) => {
            l.borrow().val == r.borrow().val
                && check(l.borrow().left.clone(), r.borrow().right.clone())
                && check(l.borrow().right.clone(), r.borrow().left.clone())
        }
        (None, None) => true,
        _ => false,
    }
}
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(x) => check(x.borrow().left.clone(), x.borrow().right.clone()),
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn is_symmetric() {
        let mn = TreeNode::new;
        assert_eq!(Solution::is_symmetric(None), true);
        assert_eq!(Solution::is_symmetric(make_node(1, None, None)), true);
        assert_eq!(Solution::is_symmetric(make_node(1, Some(mn(3)), None)), false);
        assert_eq!(Solution::is_symmetric(make_node(1, Some(mn(3)), Some(mn(3)))), true);
        assert_eq!(Solution::is_symmetric(make_node(1, Some(mn(3)), Some(mn(4)))), false);

        assert_eq!(
            Solution::is_symmetric(from_nodes(
                1,
                make_node(2, Some(mn(3)), Some(mn(4))),
                make_node(2, Some(mn(4)), Some(mn(3)))
            )),
            true
        );

        assert_eq!(
            Solution::is_symmetric(from_nodes(
                1,
                make_node(2, Some(mn(3)), Some(mn(4))),
                make_node(2, Some(mn(3)), Some(mn(4)))
            )),
            false
        );
    }
}
