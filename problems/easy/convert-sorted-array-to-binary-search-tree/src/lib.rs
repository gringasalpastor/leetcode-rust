pub struct Solution;

use treenode::*;

use std::cell::RefCell;
use std::rc::Rc;

fn make_bst_helper(slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    match slice.len() {
        0 => None,
        len => {
            let (left, right) = slice.split_at(len / 2);
            let (element, right) = right.split_first().expect("len !=0");
            from_nodes(*element, make_bst_helper(left), make_bst_helper(right))
        }
    }
}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        make_bst_helper(&nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_array_to_bst() {
        assert_eq!(Solution::sorted_array_to_bst(vec![]), None);
        assert_eq!(Solution::sorted_array_to_bst(vec![1]), make_node(1, None, None));
    }
}
