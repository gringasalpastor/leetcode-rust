pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // match nums.binary_search(&target) {
        //     Ok(x) => x as i32,
        //     Err(e) => e as i32,
        // }

        if nums.is_empty() {
            return 0;
        }

        let (mut min, mut mid, mut max) = (0i32, (nums.len() - 1) as i32 / 2, (nums.len() - 1) as i32);

        loop {
            if nums[mid as usize] == target {
                return mid; // distinct integes, no need to find first
            }
            if min >= max {
                return mid + if target > nums[mid as usize] { 1 } else { 0 };
            }

            if target < nums[mid as usize] {
                max = mid - 1;
            } else {
                min = mid + 1;
            }

            mid = (min + max) / 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_insert() {
        assert_eq!(Solution::search_insert(vec![1, 2, 3], 1), 0);
        assert_eq!(Solution::search_insert(vec![1, 2, 3], 3), 2);
        assert_eq!(Solution::search_insert(vec![], 3), 0);
        assert_eq!(Solution::search_insert(vec![1], 1), 0);
        assert_eq!(Solution::search_insert(vec![1], 3), 1);
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
        assert_eq!(Solution::search_insert(vec![1, 2, 3], 4), 3);
        assert_eq!(Solution::search_insert(vec![1, 2, 3], 0), 0);
    }
}
