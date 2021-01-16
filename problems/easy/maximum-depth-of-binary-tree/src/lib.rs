pub struct Solution;

use treenode::*;

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(n) => {
                1 + cmp::max(
                    Solution::max_depth(n.borrow().left.clone()),
                    Solution::max_depth(n.borrow().right.clone()),
                )
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_depth() {
        let mn = TreeNode::new;
        assert_eq!(Solution::max_depth(None), 0);
        assert_eq!(Solution::max_depth(make_node(1, None, None)), 1);
        assert_eq!(
            Solution::max_depth(from_nodes(
                1,
                make_node(2, Some(mn(3)), Some(mn(4))),
                make_node(2, Some(mn(4)), Some(mn(3)))
            )),
            3
        );
        assert_eq!(
            Solution::max_depth(from_nodes(1, make_node(2, Some(mn(3)), Some(mn(4))), make_node(2, Some(mn(4)), None))),
            3
        );
    }
}
