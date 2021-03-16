pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        x.reverse_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_bits() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
    }
}
