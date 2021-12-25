pub struct Solution;

impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        assert!(divisor != 0);

        let mut result_sign = 1;
        if dividend.signum() + divisor.signum() == 0 {
            result_sign = -1;
        }

        if divisor == dividend {
            return 1;
        }

        // Handle edge cases for i32::min_value() so we can convert to pos num
        if divisor == i32::min_value() {
            return 0;
        }

        let mut lost_bits = 0;
        if dividend == i32::min_value() {
            if divisor == 1 {
                return i32::min_value();
            }
            if divisor == -1 {
                return i32::max_value();
            }
            if divisor == 2 {
                return -1073741824;
            }
            if divisor == -2 {
                return 1073741824;
            }
            // If we are dividing by >= 3 then we can just use +1 to help noramlize
            dividend += 1;
            lost_bits = 1;
        }

        // Normalize as positive
        if dividend.is_negative() {
            dividend = dividend.wrapping_neg();
        }
        if divisor.is_negative() {
            divisor = divisor.wrapping_neg();
        }

        if divisor > dividend {
            return 0;
        }

        let mut result_sum = 0;

        while divisor <= dividend {
            let (mut new_divisor, mut new_divisor_prev) = (divisor, divisor);
            let (mut result, mut result_prev) = (1i32, 1i32);

            while new_divisor <= dividend {
                new_divisor_prev = new_divisor;
                result_prev = result;

                match (new_divisor.checked_shl(1), result.checked_shl(1)) {
                    (Some(x), Some(y)) if x >= 0 && y >= 0 => {
                        new_divisor = x;
                        result = y;
                    }
                    _ => {
                        break;
                    }
                }
            }
            result = result_prev;
            new_divisor = new_divisor_prev;
            result_sum += result;
            dividend = dividend - new_divisor + lost_bits;
            lost_bits = 0;
        }

        if result_sign == -1 {
            result_sum = result_sum.saturating_neg();
        }
        result_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn divide() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(-7, -3), 2);
        assert_eq!(Solution::divide(0, 1), 0);
        assert_eq!(Solution::divide(-1, 1), -1);
        assert_eq!(Solution::divide(-2, 2), -1);
        assert_eq!(Solution::divide(i32::min_value() + 1, i32::min_value()), 0);
        assert_eq!(Solution::divide(i32::min_value(), 2), -1073741824);
        assert_eq!(Solution::divide(i32::min_value(), 3), -715827882);
        assert_eq!(Solution::divide(i32::min_value(), 4), -536870912);
        assert_eq!(Solution::divide(i32::min_value(), i32::min_value()), 1);
        assert_eq!(Solution::divide(i32::max_value(), i32::max_value()), 1);
        assert_eq!(Solution::divide(i32::max_value(), i32::max_value()), 1);
    }
}
