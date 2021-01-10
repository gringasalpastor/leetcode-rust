pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut running_sum = nums[0];
        let mut best_sum = running_sum;
        for &num in nums.iter().skip(1) {
            running_sum += num;
            if num > running_sum {
                running_sum = num;
            }
            if running_sum > best_sum {
                best_sum = running_sum;
            }
        }
        best_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sub_array() {
        assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![0]), 0);
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
        assert_eq!(Solution::max_sub_array(vec![-2147483647]), -2147483647);
    }
}
