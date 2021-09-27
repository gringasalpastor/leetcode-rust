pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let target_64: i64 = target.into();

        let mut best_sum: i64 = i32::max_value().into();
        if target >= 0 {
            best_sum = i32::min_value().into();
        }

        for (i, &a) in nums.iter().enumerate().filter(|&(i, &a)| i == 0 || nums[i - 1] != a) {
            let (mut left_i, mut right_i) = (i + 1, nums.len() - 1);

            while left_i < right_i {
                let sum: i64 = (a + nums[left_i] + nums[right_i]).into();
                if (target_64 - sum).abs() < (target_64 - best_sum).abs() {
                    best_sum = sum;
                    if best_sum == target_64 {
                        return target;
                    }
                } else if sum < target_64 {
                    left_i += 1;
                } else {
                    right_i -= 1;
                }
            }
        }

        best_sum as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum_closest() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 1], 0), 3);
        assert_eq!(Solution::three_sum_closest(vec![0, 2, 1, -3], 1), 0);
    }
}
