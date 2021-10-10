pub struct Solution;

fn two_sum(nums: &Vec<i32>, target: i32, start: usize) -> Vec<Vec<i32>> {
    let (mut left, mut right) = (start, nums.len() - 1);
    let mut result: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
    while left < right {
        let cur_sum = nums[left] + nums[right];
        if left > start && nums[left] == nums[left - 1] {
            left += 1;
        } else if cur_sum < target {
            left += 1;
        } else if cur_sum > target {
            right -= 1;
        } else if right < nums.len() - 1 && nums[right] == nums[right + 1] {
            right -= 1;
        } else {
            result.push(vec![nums[right], nums[left]]);
            left += 1;
            right -= 1;
        }
    }
    result
}

fn k_sum(nums: &Vec<i32>, target: i32, start: usize, k: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
    if start == nums.len() || (nums[start] * k) > target || target > (nums.last().expect("nums is not empty") * k) {
        return result;
    }

    if k == 2 {
        return two_sum(nums, target, start);
    }

    for i in start..nums.len() {
        // if first and not a dup
        if i == start || nums[i - 1] != nums[i] {
            for mut subset in k_sum(nums, target - nums[i], i + 1, k - 1) {
                subset.push(nums[i]);
                result.push(subset);
            }
        }
    }

    result
}

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        k_sum(&nums, target, 0, 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_sum() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![2, 1, -1, -2], vec![2, 0, 0, -2], vec![1, 0, 0, -1]]
        );
    }
}
