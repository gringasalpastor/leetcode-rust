pub struct Solution;

use treenode::*;

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

fn is_balanced_helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
    match root {
        Some(node) => {
            let (depth_left, result_left) = is_balanced_helper(node.borrow().left.clone());
            let (depth_right, result_right) = is_balanced_helper(node.borrow().right.clone());
            (
                1 + cmp::max(depth_left, depth_right),
                result_right && result_left && (depth_right - depth_left).abs() <= 1,
            )
        }
        None => (0, true),
    }
}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_balanced_helper(root).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_balanced() {
        let mn = TreeNode::new;

        assert_eq!(Solution::is_balanced(None), true);
        assert_eq!(Solution::is_balanced(make_node(1, None, None)), true);
        assert_eq!(
            Solution::is_balanced(from_nodes(
                0,
                make_node(-5, Some(mn(-6)), Some(mn(2))),
                make_node(5, Some(mn(3)), Some(mn(7)))
            )),
            true
        );

        assert_eq!(
            Solution::is_balanced(from_nodes(
                0,
                make_node(-5, Some(mn(-6)), Some(mn(2))),
                make_node(5, Some(mn(3)), None)
            )),
            true
        );

        assert_eq!(
            Solution::is_balanced(from_nodes(
                1,
                from_nodes(2, make_node(4, Some(mn(8)), None), make_node(5, None, None)),
                make_node(3, Some(mn(6)), None)
            )),
            true
        );

        assert_eq!(
            Solution::is_balanced(from_nodes(0, make_node(-5, Some(mn(-6)), Some(mn(2))), make_node(5, None, None))),
            true
        );

        assert_eq!(Solution::is_balanced(from_nodes(0, make_node(-5, Some(mn(-6)), None), None)), false);
    }
}
