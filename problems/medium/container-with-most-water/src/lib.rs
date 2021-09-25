pub struct Solution;
use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left_i, mut right_i) = (0, height.len() - 1);
        let mut cur_max = 0;

        while left_i < right_i {
            cur_max = cmp::max(cur_max, (right_i - left_i) as i32 * cmp::min(height[right_i], height[left_i]));
            if height[left_i] > height[right_i] {
                right_i -= 1;
            } else {
                left_i += 1;
            }
        }
        cur_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }
}
