pub struct Solution;

use std::cmp;

pub fn largest_possible(s: &str, left_pos: usize, right_pos: usize) -> usize {
    let cur_size = right_pos - left_pos + 1;
    cur_size + 2 * cmp::min(left_pos, s.len() - right_pos + 1)
}

pub fn longest_palindrome_impl<'a>(s: &'a str, left_pos: usize, right_pos: usize, mut cur_best: &'a str) -> &'a str {
    if s.as_bytes()[left_pos] != s.as_bytes()[right_pos] {
        return cur_best;
    }
    let cur_size = right_pos - left_pos + 1;

    if cur_size > cur_best.len() {
        cur_best = &s[left_pos..right_pos + 1];
    }

    if left_pos == 0 || right_pos == s.len() - 1 {
        return cur_best;
    }

    longest_palindrome_impl(s, left_pos - 1, right_pos + 1, cur_best)
}
impl Solution {
    pub fn longest_palindrome(mut s: String) -> String {
        if s.is_empty() {
            return s;
        }

        let mid: usize = s.len() / 2;
        let mut cur_best = &s[mid..mid];

        for i in 0..mid + 1 {
            if mid + i < s.len() {
                if largest_possible(&s, mid + i, mid + i) > cur_best.len() {
                    cur_best = longest_palindrome_impl(&s, mid + i, mid + i, cur_best);
                }
                if (mid + i) != 0 {
                    if largest_possible(&s, mid + i - 1, mid + i) > cur_best.len() {
                        cur_best = longest_palindrome_impl(&s, mid + i - 1, mid + i, cur_best);
                    }
                }
            }
            if mid - i != 0 {
                if largest_possible(&s, mid - i, mid - i) > cur_best.len() {
                    cur_best = longest_palindrome_impl(&s, mid - i, mid - i, cur_best);
                }
                if (mid - i + 1) < s.len() {
                    if largest_possible(&s, mid - i, mid - i + 1) > cur_best.len() {
                        cur_best = longest_palindrome_impl(&s, mid - i, mid - i + 1, cur_best);
                    }
                }
            }
        }

        // Reuse input string instad of constructing a new string
        let start = cur_best.as_ptr() as usize - s.as_ptr() as usize;
        let end = start + cur_best.len();
        s.replace_range(..start, "");
        s.truncate(end - start);

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_palindrome() {
        let lp = Solution::longest_palindrome;
        assert_eq!(lp("".into()), "");
        assert_eq!(lp("a".into()), "a");
        assert_eq!(lp("ac".into()), "c");
        assert_eq!(lp("babad".into()), "aba");
        assert_eq!(lp("cbbd".into()), "bb");
    }
}
