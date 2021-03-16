pub struct Solution;

impl Solution {
    #![allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_ones() {
        assert_eq!(Solution::hammingWeight(0b00000000000000000000000000001011), 3);
    }
}
