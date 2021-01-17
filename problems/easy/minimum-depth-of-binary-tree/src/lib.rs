pub struct Solution;

use treenode::*;

use std::cell::RefCell;
use std::cmp;
use std::mem;
use std::rc::Rc;

fn min_depth_iterative_deepening(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> bool {
    if depth <= 0 {
        return false;
    }
    match root {
        Some(n) => match (n.borrow().left.as_ref(), n.borrow().right.as_ref()) {
            (None, None) => true,
            _ => {
                min_depth_iterative_deepening(n.borrow().left.clone(), depth - 1)
                    || min_depth_iterative_deepening(n.borrow().right.clone(), depth - 1)
            }
        },
        None => false,
    }
}

fn min_depth_helper(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
    match root {
        Some(n) => match (n.borrow().left.as_ref(), n.borrow().right.as_ref()) {
            (None, None) => depth,
            _ => cmp::min(
                min_depth_helper(n.borrow().left.clone(), depth + 1),
                min_depth_helper(n.borrow().right.clone(), depth + 1),
            ),
        },
        None => i32::max_value(),
    }
}

impl Solution {
    pub fn min_depth_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(_) => min_depth_helper(root, 1),
            None => 0,
        }
    }

    pub fn min_depth_iterative_deepening(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut i = 1;
        loop {
            if min_depth_iterative_deepening(root.clone(), i) {
                return i;
            }
            i += 1;
        }
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut nodes = vec![];
        match root {
            Some(n) => nodes.push(n),
            None => return 0,
        }

        let mut new_nodes = vec![];
        let mut i = 1;

        loop {
            for n in &nodes {
                match (n.borrow().left.as_ref(), n.borrow().right.as_ref()) {
                    (None, None) => return i,
                    (Some(l), Some(r)) => {
                        new_nodes.push(l.clone());
                        new_nodes.push(r.clone());
                    }
                    (Some(l), None) => {
                        new_nodes.push(l.clone());
                    }
                    (None, Some(r)) => {
                        new_nodes.push(r.clone());
                    }
                }
            }
            nodes.clear();
            mem::swap(&mut nodes, &mut new_nodes);

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_depth() {
        let mn = TreeNode::new;

        assert_eq!(Solution::min_depth(None), 0);
        assert_eq!(Solution::min_depth(make_node(1, None, None)), 1);
        assert_eq!(
            Solution::min_depth(from_nodes(
                0,
                make_node(-5, Some(mn(-6)), Some(mn(2))),
                make_node(5, Some(mn(3)), Some(mn(7)))
            )),
            3
        );

        assert_eq!(
            Solution::min_depth(from_nodes(
                0,
                make_node(-5, Some(mn(-6)), Some(mn(2))),
                make_node(5, Some(mn(3)), None)
            )),
            3
        );

        assert_eq!(Solution::min_depth(from_nodes(0, make_node(-5, Some(mn(-6)), Some(mn(2))), None)), 3);
        assert_eq!(Solution::min_depth(from_nodes(0, make_node(-5, Some(mn(-6)), None), None)), 3);
    }
}
