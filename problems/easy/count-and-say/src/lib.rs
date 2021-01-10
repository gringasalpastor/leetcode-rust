pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = "1".to_string();
        for _ in 0..n - 1 {
            let mut next_str = "".to_string();
            let mut previous_c = result.as_bytes()[0] as char;
            let mut count = 1;
            for c in result.chars().skip(1) {
                if c != previous_c {
                    next_str.push_str(&count.to_string());
                    next_str.push(previous_c);
                    previous_c = c;
                    count = 1;
                } else {
                    count += 1;
                }
            }
            next_str.push_str(&count.to_string());
            next_str.push(previous_c);
            result = next_str;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_and_say() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(2), "11");
        assert_eq!(Solution::count_and_say(3), "21");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}
