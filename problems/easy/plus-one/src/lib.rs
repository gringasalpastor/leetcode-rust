pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = true;
        for i in digits.iter_mut().rev() {
            if *i == 9 {
                *i = 0;
            } else {
                *i += 1;
                carry = false;
                break;
            }
        }
        if carry {
            digits.insert(0, 1);
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![1, 2, 9]), vec![1, 3, 0]);
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
    }
}
