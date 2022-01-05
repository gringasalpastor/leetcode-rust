pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let (mut left_i, mut right_i) = (0, len - 1);

        let mut pivot = 0;
        if nums[left_i] > nums[right_i] {
            while right_i - left_i >= 2 {
                let mid_i = (right_i + left_i) / 2;
                if nums[left_i] > nums[mid_i] {
                    right_i = mid_i;
                } else if nums[mid_i] > nums[right_i] {
                    left_i = mid_i;
                }
            }

            pivot = if nums[right_i] < nums[left_i] { right_i } else { left_i };
        }

        let (mut left_i, mut right_i) = (0, len - 1);

        while left_i < right_i {
            let mid_i = (right_i + left_i) / 2;

            let result_i = (mid_i + pivot) % len;
            if nums[result_i] == target {
                return result_i as i32;
            }

            if target > nums[result_i] {
                left_i = mid_i + 1;
            } else {
                right_i = mid_i.saturating_sub(1);
            }
        }

        let result_i = (left_i + pivot) % len;
        if nums[result_i] != target {
            return -1;
        }
        result_i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 7), 3);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![1, 3], 1), 0);
        assert_eq!(Solution::search(vec![1], 1), 0);
        assert_eq!(Solution::search(vec![5, 1, 3], 1), 1);
        assert_eq!(Solution::search(vec![1, 3, 5], 0), -1);
        assert_eq!(Solution::search(vec![2, 3, 7, 8, 1], 5), -1);
    }
}
