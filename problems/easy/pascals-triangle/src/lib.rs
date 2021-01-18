pub struct Solution;

use std::iter;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {
            return Vec::<Vec<i32>>::new();
        }

        let mut result = vec![vec![1]];
        for i in 0..(num_rows - 1) as usize {
            let mut new_row = Vec::<i32>::new();
            // NOTE: We could avoid the copy if we could use `Itertools::tuple_windows`
            for slice in iter::once(&0).chain(&result[i]).chain(iter::once(&0)).collect::<Vec<_>>().windows(2) {
                new_row.push(slice[0] + slice[1]);
            }
            result.push(new_row);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        assert_eq!(Solution::generate(0), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::generate(1), [[1]]);
        assert_eq!(Solution::generate(2), [vec![1], vec![1, 1]]);
        assert_eq!(Solution::generate(3), [vec![1], vec![1, 1], vec![1, 2, 1]]);
        assert_eq!(Solution::generate(4), [vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]);
    }
}
