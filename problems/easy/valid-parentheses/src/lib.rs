pub struct Solution;

fn lookup_open(c: char) -> char {
    match c {
        ')' => '(',
        '}' => '{',
        ']' => '[',
        _ => '_',
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = String::new();
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => stack.push(c),
                ')' | '}' | ']' => {
                    if stack.pop().unwrap_or('_') != lookup_open(c) {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_valid() {
        assert_eq!(Solution::is_valid("".into()), true);
        assert_eq!(Solution::is_valid("()".into()), true);
        assert_eq!(Solution::is_valid("()[]{}".into()), true);
        assert_eq!(Solution::is_valid("(]".into()), false);
        assert_eq!(Solution::is_valid("([)]".into()), false);
        assert_eq!(Solution::is_valid("{[]}".into()), true);
    }
}
