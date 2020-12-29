pub struct Solution;

use std::iter::Peekable;

fn lookup_single(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

fn lookup_pair<I: Iterator<Item = char>>(letters: (char, char), iter: Peekable<I>) -> i32 {
    (match letters {
        ('I', 'V') => -1,
        ('I', 'X') => -1,
        ('X', 'L') => -10,
        ('X', 'C') => -10,
        ('C', 'D') => -100,
        ('C', 'M') => -100,
        _ => lookup_single(letters.0),
    } + roman_to_int_helper(iter) )
}

fn roman_to_int_helper<I: Iterator<Item = char>>(mut iter: Peekable<I>) -> i32 {
    let c = iter.next().unwrap_or('_');
    if let Some(&c_next) = iter.peek() {
        return lookup_pair((c, c_next), iter);
    } else {
        return lookup_single(c);
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        roman_to_int_helper(s.chars().peekable())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_to_int() {
        assert_eq!(Solution::roman_to_int("".to_string()), 0);
        assert_eq!(Solution::roman_to_int("I".to_string()), 1);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
