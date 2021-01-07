pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        for current_pos in 0..haystack.len() {
            let end = current_pos + needle.len();
            if end > haystack.len() {
                return -1;
            }
            if haystack[current_pos..end] == needle {
                return current_pos as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn str_str() {
        assert_eq!(Solution::str_str("hello".into(), "ll".into()), 2);
        assert_eq!(Solution::str_str("aaaa".into(), "bba".into()), -1);
        assert_eq!(Solution::str_str("".into(), "".into()), 0);
        assert_eq!(Solution::str_str("h".into(), "".into()), 0);
        assert_eq!(Solution::str_str("".into(), "h".into()), -1);
    }
}
