pub struct Solution;

fn calc_product(i: i32, j: i32) -> i128 {
    let mut result: i128 = 1;
    for x in i..j {
        result *= x as i128;
    }
    return result;
}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let m = row_index + 1;

        let mut result = vec![0; m as usize];
        for n in 0..(row_index / 2 + 1) {
            let val = (calc_product(m - n, m) / calc_product(1, n + 1)) as i32;
            result[n as usize] = val;
            result[(m - 1 - n) as usize] = val;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_row() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
        assert_eq!(
            Solution::get_row(33),
            vec![
                1, 33, 528, 5456, 40920, 237336, 1107568, 4272048, 13884156, 38567100, 92561040, 193536720, 354817320,
                573166440, 818809200, 1037158320, 1166803110, 1166803110, 1037158320, 818809200, 573166440, 354817320,
                193536720, 92561040, 38567100, 13884156, 4272048, 1107568, 237336, 40920, 5456, 528, 33, 1
            ]
        );
    }
}
