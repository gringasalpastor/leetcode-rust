pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
use treenode::*;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return match (p, q) {
            (Some(x), Some(y)) => {
                x.borrow().val == y.borrow().val
                    && Solution::is_same_tree(x.borrow().left.clone(), y.borrow().left.clone())
                    && Solution::is_same_tree(x.borrow().right.clone(), y.borrow().right.clone())
            }
            (None, None) => true,
            _ => false,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_same_tree() {
        let mn = TreeNode::new;
        assert_eq!(Solution::is_same_tree(None, None), true);
        assert_eq!(Solution::is_same_tree(make_node(1, None, None), make_node(1, None, None)), true);
        assert_eq!(Solution::is_same_tree(make_node(1, None, None), make_node(2, None, None)), false);
        assert_eq!(Solution::is_same_tree(make_node(1, Some(mn(3)), None), None), false);

        let a = make_node(1, Some(mn(3)), None);
        let b = make_node(1, Some(mn(3)), None);
        assert_eq!(Solution::is_same_tree(a, b), true);

        let a = make_node(1, Some(mn(1)), None);
        let b = make_node(1, Some(mn(2)), None);
        assert_eq!(Solution::is_same_tree(a, b), false);
    }
}
