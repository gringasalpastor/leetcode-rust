pub struct Solution;

impl Solution {
    pub fn num_digits(x: u32) -> u8 {
        let (mut min, mut max) = (1u8, 10u8);
        let mut cur: u8 = (min + max) / 2;

        while min != max {
            if x / 10_u32.pow(cur.into()) > 0 {
                min = cur + 1;
            } else {
                max = cur;
            }

            cur = (min + max) / 2;
        }

        return cur;
    }

    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        let x = x as u32;

        let digits = Solution::num_digits(x);

        for i in 0..digits / 2 + 1 {
            let right = x / 10_u32.pow(i.into()) % 10;
            let left = x / 10_u32.pow((digits - i - 1).into()) % 10;
            if left != right {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_digits() {
        assert_eq!(Solution::num_digits(0), 1);
        assert_eq!(Solution::num_digits(1), 1);
        assert_eq!(Solution::num_digits(2), 1);
        assert_eq!(Solution::num_digits(10), 2);
        assert_eq!(Solution::num_digits(111), 3);
        assert_eq!(Solution::num_digits(2147483647), 10);
    }

    #[test]
    fn is_palindrome() {
        assert_eq!(Solution::is_palindrome(-1), false);
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(1), true);
        assert_eq!(Solution::is_palindrome(11), true);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(12), false);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(1121), false);
        assert_eq!(Solution::is_palindrome(1221), true);
    }
}
