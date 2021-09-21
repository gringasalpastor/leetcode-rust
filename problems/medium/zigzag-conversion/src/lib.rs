pub struct Solution;

impl Solution {
    pub fn convert(mut s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut result: Vec<Vec<char>> = vec![Vec::new(); num_rows as usize];

        let (mut row, mut step): (usize, i32) = (0, 1);
        for c in s.chars() {
            result[row].push(c);
            if row == num_rows as usize - 1 {
                step = -1;
            }
            if row == 0 {
                step = 1;
            }

            row = (row as i32 + step) as usize;
        }
        s = result.iter().flatten().collect();
        return s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert() {
        assert_eq!(Solution::convert("PAYPALISHIRING".into(), 3), "PAHNAPLSIIGYIR");
        assert_eq!(Solution::convert("PAYPALISHIRING".into(), 1), "PAYPALISHIRING");
        assert_eq!(Solution::convert("PAYPALISHIRING".into(), 4), "PINALSIGYAHRPI");
    }
}
