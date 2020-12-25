pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // Leetcode is using rust 1.40 so we can't use `i32::MAX`
        if x == i32::min_value() {
            return 0;
        }

        let sign = x.signum();
        let mut x = x * sign;

        let mut result = x % 10;
        x /= 10;
        while x > 0 {
            let (result_, overflow) = result.overflowing_mul(10);
            if overflow {
                return 0;
            }
            /*
                Note: This add can't overflow because right most digit of `i32::MAX` is `7`
                which makes inputs like `8463847412` impossible (rev of 32::MAX + 1).
            */
            result = result_ + x % 10;

            x /= 10;
        }
        return result * sign;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(1), 1);

        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(-1), -1);
    }
    #[test]
    fn edge_cases() {
        assert_eq!(Solution::reverse(i32::MIN), 0);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(i32::MAX), 0);
        // assert_eq!(Solution::reverse(8463847412), 0); not possible
    }
}
