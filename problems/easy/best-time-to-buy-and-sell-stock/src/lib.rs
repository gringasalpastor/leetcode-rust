pub struct Solution;

use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_prof = 0;
        let mut cur_min_price = i32::max_value();

        for price in prices {
            cur_min_price = cmp::min(price, cur_min_price);
            max_prof = cmp::max(max_prof, price - cur_min_price);
        }
        max_prof
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}
