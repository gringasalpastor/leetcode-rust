pub struct Solution;
use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut cur_length = 0;

        // Index of last place we saw this char
        let mut visited: [i32; 128] = [-1; 128];

        for (cur_index, c) in s.chars().enumerate() {
            let visited_index = visited[c as usize];

            if cur_index as i32 <= cur_length + visited_index  {
                cur_length = cur_index as i32 - visited_index - 1;
            }
            cur_length += 1;
            longest = cmp::max(cur_length, longest);

            visited[c as usize] = cur_index as i32;
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
