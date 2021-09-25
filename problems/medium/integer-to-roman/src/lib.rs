pub struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let roman_digits = vec![
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
        ];

        let mut cur_digit_index = roman_digits.len() - 1;

        let mut result: Vec<&str> = vec![];

        while num != 0 {
            if num >= roman_digits[cur_digit_index].0 {
                result.push(roman_digits[cur_digit_index].1);
                num -= roman_digits[cur_digit_index].0;
            } else {
                cur_digit_index -= 1;
            }
        }

        result.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn int_to_roman() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
