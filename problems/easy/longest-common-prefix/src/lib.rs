pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".into();
        }

        let mut shortest = strs.swap_remove(
            strs.iter()
                .enumerate()
                .min_by(|a, b| a.1.len().cmp(&b.1.len()))
                .expect("list not empty")
                .0,
        );

        for i in 0..shortest.len() {
            for s in &strs {
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
