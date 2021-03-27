pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        for i in nums {
            if !seen.insert(i) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    use super::*;
    fn contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4, 3]), true);
    }
}
