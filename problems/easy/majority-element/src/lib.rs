pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        const BITS: usize = std::mem::size_of::<i32>() * 8;
        let mut num_bits: [i32; BITS] = [0; BITS];
        let majority = (nums.len() as i32 / 2) + 1;
        for i in nums {
            for bit_pos in 0..BITS {
                num_bits[bit_pos] += (i >> bit_pos) & 1;
            }
        }

        let mut res = 0;

        for bit_pos in 0..BITS {
            if num_bits[bit_pos] >= majority {
                res |= 1 << bit_pos;
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
