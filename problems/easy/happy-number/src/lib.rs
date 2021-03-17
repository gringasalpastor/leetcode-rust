pub struct Solution;

use std::collections::HashSet;

fn sum_of_squares(mut x: i32) -> i32 {
    let mut sum = 0;
    while x != 0 {
        sum += (x % 10).pow(2);
        x /= 10;
    }
    sum
}

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut previous = HashSet::new();
        loop {
            if n == 1 {
                return true;
            }
            if previous.contains(&n) {
                return false;
            }
            previous.insert(n.clone());
            n = sum_of_squares(n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_happy() {
        assert!(Solution::is_happy(19));
        assert!(!Solution::is_happy(2));
        assert!(Solution::is_happy(7));
    }
}
