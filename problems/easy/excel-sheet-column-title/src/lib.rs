pub struct Solution;

fn digit_to_char(d: u8) -> char {
    return ('A' as u8 + d - 1) as char;
}
fn get_diget(i: u32, pos: u32) -> u32 {
    let mut digit = ((i as u64 % 26_u64.pow(pos + 1)) / 26_u64.pow(pos)) as u32;
    if digit == 0 {
        digit = 26;
    }
    digit
}
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut num = column_number as u32;
        let mut result = String::new();
        let mut pos = 0_u32;
        loop {
            let digit = get_diget(num, pos);
            result.push(digit_to_char(digit as u8));

            num -= 26_u32.pow(pos) * digit;
            pos += 1;
            if num <= 0 {
                break;
            }
        }

        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert_to_title() {
        assert_eq!(Solution::convert_to_title(1), "A");
        assert_eq!(Solution::convert_to_title(28), "AB");
        assert_eq!(Solution::convert_to_title(2147483647), "FXSHRXW");
        assert_eq!(Solution::convert_to_title(52), "AZ");
    }
}
