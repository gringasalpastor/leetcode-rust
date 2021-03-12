pub struct Solution;

impl Solution {
    pub fn max_profit(mut prices: Vec<i32>) -> i32 {
        let mut prof = 0;

        if prices.len() <= 1 {
            return 0;
        }
        prices.push(i32::min_value()); // Force sell at the end
        let mut cur_buy_price = prices[0];
        let mut increasing = true;

        for win in prices.windows(2) {
            if (win[1] == win[0]) || (increasing && win[1] > win[0]) || (!increasing && win[1] < win[0]) {
                continue;
            }

            if increasing && win[1] < win[0] {
                prof += win[0] - cur_buy_price;
                increasing = false;
            }
            if !increasing && win[1] > win[0] {
                cur_buy_price = win[0];
                increasing = true;
            }
        }
        prof
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 0, 4]), 4);
    }
}
