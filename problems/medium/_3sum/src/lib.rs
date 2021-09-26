pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(nums.len() / 3);

        nums.sort_unstable();

        for (i, &a) in nums.iter().enumerate() {
            if nums[i] > 0 {
                break;
            }
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, nums.len() - 1);

            while left < right {
                let sum = a + nums[left] + nums[right];
                if sum == 0 {
                    result.push(vec![a, nums[left], nums[right]]);
                    left += 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    right -= 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum() {
        assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(
            Solution::three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]),
            vec![vec![-4, -2, 6], vec![-4, 0, 4], vec![-4, 1, 3], vec![-4, 2, 2], vec![-2, -2, 4], vec![-2, 0, 2]]
        );
    }
}
