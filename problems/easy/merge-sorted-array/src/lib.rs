pub struct Solution;

impl Solution {
    pub fn merge(v1: &mut Vec<i32>, m: i32, v2: &mut Vec<i32>, n: i32) {
        let mut v1_index: usize = 0;
        let mut v2_index: usize = 0;

        for i in 0..(m + n) {
            let val1 = if v1_index < m as usize { v1[v1_index] } else { i32::max_value() };
            let val2 = if v2_index < n as usize { v2[v2_index] } else { i32::max_value() };
            if val1 < val2 {
                v1[((i + m) % (m + n)) as usize] = val1;
                v1_index += 1;
            } else {
                v1[((i + m) % (m + n)) as usize] = val2;
                v2_index += 1;
            }
        }
        v1.rotate_left(m as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge() {
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![1, 2, 3];
        v1.resize_with(v1.len() + v2.len(), Default::default);
        let size1 = (v1.len() - v2.len()) as i32;
        let size2 = v2.len() as i32;

        Solution::merge(&mut v1, size1, &mut v2, size2);
        assert_eq!(v1, [1, 1, 2, 2, 3, 3]);
    }
}
