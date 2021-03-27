pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen = HashMap::new();
        for (index, val) in nums.iter().enumerate() {
            if i64::wrapping_abs(index as i64 - *seen.get(val).unwrap_or(&i32::min_value()) as i64) <= k as i64 {
                return true;
            }
            seen.insert(val, index as i32);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contains_nearby_duplicate() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
    }
}
