pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut is_prime: Vec<bool> = vec![true; n];

        if n < 3 {
            return 0;
        }

        let mut sum = 1;

        for x in (3..n).step_by(2) {
            if is_prime[x] {
                sum += 1;
                for i in (x..n).step_by(x) {
                    is_prime[i] = false;
                }
            }
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_primes() {
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(0), 0);
        assert_eq!(Solution::count_primes(1), 0);
        assert_eq!(Solution::count_primes(2), 0);
        assert_eq!(Solution::count_primes(3), 1);
        assert_eq!(Solution::count_primes(499979), 41537);
    }
}
