pub struct Solution;

use treenode::*;

use std::cell::RefCell;
use std::rc::Rc;

fn inorder_traversal_helper(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(n) => {
            inorder_traversal_helper(n.borrow().left.clone(), result);
            result.push(n.borrow().val);
            inorder_traversal_helper(n.borrow().right.clone(), result);
        }
        None => {}
    }
}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        inorder_traversal_helper(root, &mut result);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inorder_traversal() {
        let mn = TreeNode::new;
        assert_eq!(Solution::inorder_traversal(None), []);
        assert_eq!(Solution::inorder_traversal(make_node(1, None, None)), [1]);

        assert_eq!(Solution::inorder_traversal(from_nodes(1, None, make_node(2, Some(mn(3)), None))), [1, 3, 2]);
    }
}
