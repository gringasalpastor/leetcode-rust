pub struct Solution;
use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut cur_length = 0;

        let mut visited: [bool; 128] = [false; 128];

        for c in s.chars() {
            if visited[c as usize] {
                cur_length = 0;
            }
            cur_length += 1;
            longest = cmp::max(cur_length, longest);

            visited[c as usize] = true;
        }

        return longest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
        assert_eq!(Solution::length_of_longest_substring("".into()), 0);
        assert_eq!(Solution::length_of_longest_substring("dvdf".into()), 3);
    }
}
