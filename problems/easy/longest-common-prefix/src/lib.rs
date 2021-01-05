pub struct Solution;

use std::mem;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        let mut shortest = mem::replace(
            strs.iter_mut()
                .min_by(|a, b| a.len().cmp(&b.len()))
                .unwrap_or(&mut "".into()),
            String::new(),
        );

        for i in 0..shortest.len() {
            for s in strs.iter().filter(|&x| !x.is_empty()) {
                if s.as_bytes()[i] != shortest.as_bytes()[i] {
                    shortest.truncate(i);
                    return shortest;
                }
            }
        }

        shortest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_common_prefix() {
        fn svec(strings: &[&str]) -> Vec<String> {
            strings.iter().map(|&s| s.into()).collect()
        }

        assert_eq!(Solution::longest_common_prefix(vec![]), "");
        assert_eq!(Solution::longest_common_prefix(svec(&["", ""])), "");
        assert_eq!(Solution::longest_common_prefix(svec(&["a", "ab"])), "a");
        assert_eq!(Solution::longest_common_prefix(svec(&["a", "ab", "xyz"])), "");
        assert_eq!(Solution::longest_common_prefix(svec(&["a", "", "xyz"])), "");
        assert_eq!(Solution::longest_common_prefix(svec(&["ab", "ab", "abc"])), "ab");
        assert_eq!(
            Solution::longest_common_prefix(svec(&["flower", "flow", "flight"])),
            "fl"
        );
        assert_eq!(Solution::longest_common_prefix(svec(&["dog", "racecar", "car"])), "");
    }
}
