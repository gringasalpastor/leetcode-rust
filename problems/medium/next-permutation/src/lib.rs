pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }

        let last_i = nums.len() - 1;

        let mut cur_i: i32 = (last_i - 1) as i32;
        while cur_i >= 0 {
            if nums[cur_i as usize] < nums[(cur_i + 1) as usize] {
                break;
            }
            cur_i -= 1;
        }
        if cur_i < 0 {
            // we are at largest, reset to smallest
            nums.reverse();
        } else {
            // Find next largest
            let mut next_largest_i = last_i;
            while nums[cur_i as usize] >= nums[next_largest_i] {
                next_largest_i -= 1;
            }

            nums.swap(cur_i as usize, next_largest_i as usize);
            let (_, right) = nums.split_at_mut(cur_i as usize + 1);
            right.sort()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn next_permutation() {
        let test = |nums: &[i32]| -> Vec<i32> {
            let mut result = nums.to_vec();
            Solution::next_permutation(&mut result);
            result
        };

        assert_eq!(test(&[1, 2, 3]), [1, 3, 2]);
        assert_eq!(test(&[3, 2, 1]), [1, 2, 3]);
        assert_eq!(test(&[1, 1, 5]), [1, 5, 1]);
        assert_eq!(test(&[2, 20, 1]), [20, 1, 2]);
        assert_eq!(test(&[1, 7, 6, 5, 4]), [4, 1, 5, 6, 7]);
    }
}
