pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let trimmed = s.trim_start_matches(' ');
        if trimmed.is_empty() {
            return 0;
        }
        let is_neg = trimmed.as_bytes()[0] == b'-';
        let first_digit_index = if trimmed.as_bytes()[0] == b'+' || trimmed.as_bytes()[0] == b'-' { 1 } else { 0 };

        let search_str = &trimmed[first_digit_index..];
        let last_digit_index =
            search_str.find(|c: char| !c.is_ascii_digit()).unwrap_or(search_str.len()) + first_digit_index;

        if last_digit_index - first_digit_index == 0 {
            return 0;
        }

        trimmed[0..last_digit_index].parse::<i32>().unwrap_or_else(|_| {
            if is_neg {
                i32::min_value()
            } else {
                i32::max_value()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atoi() {
        assert_eq!(Solution::my_atoi("".into()), 0);
        assert_eq!(Solution::my_atoi(" ".into()), 0);
        assert_eq!(Solution::my_atoi(" +".into()), 0);
        assert_eq!(Solution::my_atoi(" -".into()), 0);
        assert_eq!(Solution::my_atoi("+".into()), 0);
        assert_eq!(Solution::my_atoi("-".into()), 0);
        assert_eq!(Solution::my_atoi("+0d".into()), 0);
        assert_eq!(Solution::my_atoi("-0d".into()), 0);
        assert_eq!(Solution::my_atoi("-1d".into()), -1);
        assert_eq!(Solution::my_atoi("-1".into()), -1);
        assert_eq!(Solution::my_atoi("2".into()), 2);
        assert_eq!(Solution::my_atoi("2 ".into()), 2);
        assert_eq!(Solution::my_atoi(" 2 ".into()), 2);
        assert_eq!(Solution::my_atoi(" +2 ".into()), 2);
        assert_eq!(Solution::my_atoi(" -2 ".into()), -2);
        assert_eq!(Solution::my_atoi("d".into()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".into()), -2147483648);
        assert_eq!(Solution::my_atoi("91283472332".into()), 2147483647);
    }
}
