use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for (index, val) in nums.iter().enumerate() {
            let solution = target - val;
            if seen.contains_key(&solution) {
                return vec![seen[&solution] as i32, index as i32];
            }
            seen.insert(*val, index);
        }
        panic!("no possible solution");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![2, 3, 7, 11], 9), vec![0, 2]);
    }

    #[test]
    #[should_panic]
    fn bad_input() {
        Solution::two_sum(vec![2, 3, 7, 11], 99);
    }

    #[test]
    #[should_panic]
    fn empty() {
        Solution::two_sum(vec![], 99);
    }
}
