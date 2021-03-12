pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut rng = s.chars().filter(|x| x.is_alphanumeric()).map(|x| x.to_ascii_lowercase());
        let mut rng_rev = s.chars().rev().filter(|x| x.is_alphanumeric()).map(|x| x.to_ascii_lowercase());

        while let (Some(a), Some(b)) = (rng.next(), rng_rev.next()) {
            if a != b {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_palindrome() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".into()), true);
        assert_eq!(Solution::is_palindrome("race a car".into()), false);
    }
}
