pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::<String>::new();
        }
        let num_4_letter: usize = digits.chars().filter(|&x| x == '7' || x == '9').count();

        let size = 3_usize.pow((digits.len() - num_4_letter) as u32) * 4_usize.pow(num_4_letter as u32);

        let mut result = Vec::with_capacity(size);
        result.push(String::new());

        for l in digits.chars() {
            let letters = match l {
                '2' => "abc",
                '3' => "def",
                '4' => "ghi",
                '5' => "jkl",
                '6' => "mno",
                '7' => "pqrs",
                '8' => "tuv",
                '9' => "wxyz",
                _ => panic!("Invalid digit"),
            };
            let size = result.len();
            // Requires 1.53 :)
            // for _ in 0..letters.len() - 1 {
            //     result.extend_from_within(0..size);
            // }

            result.resize_with(size * letters.len(), Default::default);

            for i in size..size * letters.len() {
                result[i] = result[i % size].clone();
            }

            for (c_i, c) in letters.chars().enumerate() {
                for i in 0 + (size * c_i)..size + (size * c_i) {
                    result[i].push(c);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_combinations() {
        let mut res = Solution::letter_combinations("23".into());
        res.sort();
        assert_eq!(res, vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
        res = Solution::letter_combinations("9".into());
        res.sort();
        assert_eq!(res, vec!["w", "x", "y", "z"]);
        res = Solution::letter_combinations("29".into());
        res.sort();
        assert_eq!(res, vec!["aw", "ax", "ay", "az", "bw", "bx", "by", "bz", "cw", "cx", "cy", "cz",]);

        res = Solution::letter_combinations("".into());
        assert_eq!(res, Vec::<String>::new());
    }
}
