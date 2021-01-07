pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut insert_pos = 0;

        for current_pos in 1..nums.len() {
            if nums[insert_pos as usize] != nums[current_pos as usize] {
                insert_pos += 1;
                nums[insert_pos as usize] = nums[current_pos as usize];
            }
        }

        insert_pos + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_duplicates() {
        let rd = Solution::remove_duplicates;

        let mut v = vec![];
        assert_eq!(rd(&mut v), 0);
        assert_eq!(v, []);

        let mut v = vec![1];
        assert_eq!(rd(&mut v), 1);
        assert_eq!(v, [1]);

        let mut v = vec![1, 1];
        assert_eq!(rd(&mut v), 1);
        assert_eq!(v, [1, 1]);

        let mut v = vec![1, 1, 1];
        assert_eq!(rd(&mut v), 1);
        assert_eq!(v, [1, 1, 1]);

        let mut v = vec![1, 1, 1, 2];
        assert_eq!(rd(&mut v), 2);
        assert_eq!(v, [1, 2, 1, 2]);

        let mut v = vec![1, 1, 1, 2, 2];
        assert_eq!(rd(&mut v), 2);
        assert_eq!(v, [1, 2, 1, 2, 2]);

        let mut v = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(rd(&mut v), 3);
        assert_eq!(v, [1, 2, 3, 2, 2, 3]);
    }
}
