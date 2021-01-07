pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut insert_pos = 0;
        for current_pos in 0..nums.len() {
            if nums[current_pos as usize] != val {
                nums[insert_pos as usize] = nums[current_pos as usize];
                insert_pos += 1;
            }
        }

        insert_pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_element() {
        #[track_caller]
        fn check(v: &mut Vec<i32>, val: i32, expected_slice: &[i32]) {
            assert_eq!(Solution::remove_element(v, val), expected_slice.len() as i32);
            assert_eq!(&v[..expected_slice.len() as usize], expected_slice);
        }

        check(&mut vec![3, 2, 2, 3], 3, &[2, 2]);
        check(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2, &[0, 1, 3, 0, 4]);
        check(&mut vec![], 2, &[]);
        check(&mut vec![1], 2, &[1]);
        check(&mut vec![2], 2, &[]);
    }
}
