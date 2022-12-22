pub struct Solution;

pub fn expand_left(nums: &Vec<i32>, mut left: usize, mut right: usize, target: i32) -> i32 {
    // Avoid having left == 0 so we can always index into previuos
    if left == 0 {
        if nums[0] == target {
            return 0;
        }
    } else {
        left = 1;
    }

    while left < right {
        let mid = (left + right) / 2;
        if nums[mid] == target && nums[mid - 1] != target {
            return mid as i32;
        } else if nums[mid] == target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    right as i32
}

pub fn expand_right(nums: &Vec<i32>, mut left: usize, mut right: usize, target: i32) -> i32 {
    // Avoid issue with indexing past the end
    let end_i = nums.len() - 1;
    if right == end_i {
        if nums[end_i] == target {
            return end_i as i32;
        }
    } else {
        right = end_i - 1;
    }

    while left < right {
        let mid = (left + right) / 2;
        if nums[mid] == target && nums[mid + 1] != target {
            return mid as i32;
        } else if nums[mid] == target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    right as i32
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let (mut left, mut right) = (0, nums.len() - 1);

        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] == target {
                return vec![expand_left(&nums, left, mid, target), expand_right(&nums, mid, right, target)];
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if nums[left] == target {
            return vec![left as i32, left as i32];
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_range() {
        assert_eq!(Solution::search_range(vec![5], 5), vec![0, 0]);
        assert_eq!(Solution::search_range(vec![5], 6), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![], 8), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![0, 1, 2, 3, 4, 4, 4], 2), vec![2, 2]);
        assert_eq!(Solution::search_range(vec![1, 2, 3, 3, 3, 3, 4, 5, 9], 3), vec![2, 5]);
        assert_eq!(Solution::search_range(vec![1, 1, 3, 4, 5, 5, 5, 5, 5], 3), vec![2, 2]);
    }
}
