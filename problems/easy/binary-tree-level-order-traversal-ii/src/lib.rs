pub struct Solution;

use treenode::*;

use std::cell::RefCell;
use std::rc::Rc;

fn level_order_helper(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Vec<i32>>, level: usize) {
    match root {
        Some(n) => {
            if result.len() == level {
                result.push(vec![]);
            }
            result[level].push(n.borrow().val);
            level_order_helper(n.borrow().left.clone(), result, level + 1);
            level_order_helper(n.borrow().right.clone(), result, level + 1);
        }
        None => {}
    }
}

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();

        level_order_helper(root, &mut result, 0);
        result.reverse();

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_order_bottom() {
        let mn = TreeNode::new;
        assert_eq!(Solution::level_order_bottom(None), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::level_order_bottom(make_node(1, None, None)), [[1]]);
        assert_eq!(
            Solution::level_order_bottom(from_nodes(
                1,
                make_node(2, Some(mn(3)), Some(mn(4))),
                make_node(2, Some(mn(4)), Some(mn(3)))
            )),
            [vec![3, 4, 4, 3], vec![2, 2], vec![1]]
        );
    }
}
