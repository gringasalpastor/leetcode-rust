pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut min = 0;
        let mut max = x;

        let delta = max - min;
        let mut delta = delta >> (32 - delta.leading_zeros()) / 2;
        let mut guess = min + delta;

        while min < max {
            let (square, overflow) = guess.overflowing_pow(2);
            if square > x || overflow {
                max = guess - 1;
            } else {
                min = guess;
            }
            guess = min + 1.max((max - min) / 2);
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn my_sqrt() {
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(2), 1);
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(i32::max_value()), 46340);
    }
}
