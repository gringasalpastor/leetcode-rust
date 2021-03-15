pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut sum = 0;

        for (i, c) in column_title.chars().rev().enumerate() {
            let digit = (c as u8 - 'A' as u8) + 1;
            sum += 26_i32.pow(i as u32) * digit as i32;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn title_to_number() {
        assert_eq!(Solution::title_to_number("A".into()), 1);
        assert_eq!(Solution::title_to_number("AB".into()), 28);
        assert_eq!(Solution::title_to_number("AZ".into()), 52);
        assert_eq!(Solution::title_to_number("ZY".into()), 701);
        assert_eq!(Solution::title_to_number("FXSHRXW".into()), 2147483647);
    }
}
