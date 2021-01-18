pub struct Solution;

use treenode::*;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            Some(n) => match (n.borrow().left.as_ref(), n.borrow().right.as_ref()) {
                (None, None) => (sum - n.borrow().val) == 0,
                _ => {
                    Solution::has_path_sum(n.borrow().left.clone(), sum - n.borrow().val)
                        || Solution::has_path_sum(n.borrow().right.clone(), sum - n.borrow().val)
                }
            },
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_path_sum() {
        let mn = TreeNode::new;
        assert_eq!(Solution::has_path_sum(None, 0), false);
        assert_eq!(Solution::has_path_sum(make_node(1, None, None), 1), true);
        assert_eq!(Solution::has_path_sum(make_node(1, None, None), 2), false);

        assert_eq!(
            Solution::has_path_sum(
                from_nodes(0, make_node(-5, Some(mn(-6)), Some(mn(2))), make_node(5, Some(mn(3)), Some(mn(7)))),
                -11
            ),
            true
        );

        assert_eq!(
            Solution::has_path_sum(
                from_nodes(0, make_node(-5, Some(mn(-6)), Some(mn(2))), make_node(5, Some(mn(3)), Some(mn(7)))),
                -3
            ),
            true
        );
    }
}
