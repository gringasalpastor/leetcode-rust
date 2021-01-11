use std::iter;

pub struct Solution;

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let (longest, shortest) = if a.len() >= b.len() { (&mut a, &mut b) } else { (&mut b, &mut a) };

        let mut l = String::new();
        std::mem::swap(&mut l, longest);
        let mut longest_bytes = l.into_bytes();

        let mut carry: u8 = 0;
        for (&a_, b_) in shortest.as_bytes().iter().rev().chain(iter::repeat(&0)).zip(longest_bytes.iter_mut().rev()) {
            let sum = a_.overflowing_add(*b_).0.overflowing_add(carry).0;

            *b_ = (*b_ & !1) + (sum & 1);
            carry = (sum >> 1) & 1;
        }
        if carry == 1 {
            longest_bytes.insert(0, '1' as u8);
        }

        // Reuse the existing `Vec` allocation cheaper than `from_utf8_lossy`
        // return String::from_utf8_lossy(&longest_bytes).into_owned();
        unsafe { String::from_utf8_unchecked(longest_bytes) }.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_binary() {
        assert_eq!(Solution::add_binary("1010".into(), "1011".into()), "10101");
        assert_eq!(Solution::add_binary("1".into(), "111".into()), "1000");
        assert_eq!(Solution::add_binary("0100".into(), "001".into()), "0101");
        assert_eq!(Solution::add_binary("0111".into(), "001".into()), "1000");
        assert_eq!(Solution::add_binary("11".into(), "10".into()), "101");
        assert_eq!(Solution::add_binary("1".into(), "0".into()), "1");
        assert_eq!(Solution::add_binary("1".into(), "".into()), "1");
        assert_eq!(Solution::add_binary("0".into(), "1".into()), "1");
        assert_eq!(Solution::add_binary("1".into(), "1".into()), "10");
    }
}
