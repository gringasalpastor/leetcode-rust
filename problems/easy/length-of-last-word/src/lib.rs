pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_ascii_whitespace().last().unwrap_or_default().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".into()), 5);
        assert_eq!(Solution::length_of_last_word(" ".into()), 0);
    }
}
