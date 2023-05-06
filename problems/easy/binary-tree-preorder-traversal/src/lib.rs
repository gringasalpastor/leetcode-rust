pub struct Solution;

use treenode::*;

use std::cell::RefCell;
use std::rc::Rc;

fn preorder_traversal_helper(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(n) => {
            result.push(n.borrow().val);
            preorder_traversal_helper(n.borrow().left.clone(), result);
            preorder_traversal_helper(n.borrow().right.clone(), result);
        }
        None => {}
    }
}

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        preorder_traversal_helper(root, &mut result);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preorder_traversal() {
        let mn = TreeNode::new;
        assert_eq!(Solution::preorder_traversal(None), []);
        assert_eq!(Solution::preorder_traversal(make_node(1, None, None)), [1]);

        assert_eq!(Solution::preorder_traversal(from_nodes(1, None, make_node(2, Some(mn(3)), None))), [1, 2, 3]);
    }
}
