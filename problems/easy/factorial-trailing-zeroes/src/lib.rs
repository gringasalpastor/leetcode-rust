pub struct Solution;

fn reduce_by(mut x: i32, factor: i32) -> i32 {
    let mut count = 0;
    loop {
        if x % factor == 0 {
            x /= 5;
            count += 1;
        } else {
            return count;
        }
    }
}

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut result = 0;
        for i in 1..n + 1 {
            result += reduce_by(i, 5);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
        assert_eq!(Solution::trailing_zeroes(0), 0);
    }
}
