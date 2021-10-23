pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut cur_strs = (Vec::<String>::with_capacity(1 << n), Vec::<(i32, i32)>::with_capacity(1 << n));
        cur_strs.0.push(String::with_capacity(n as usize * 2));
        cur_strs.1.push((0, 0));

        for _ in 0..n * 2 {
            for i in 0..cur_strs.0.len() {
                let num_open = cur_strs.1[i].0;
                let num_close = cur_strs.1[i].1;

                if num_close < num_open && num_close != n {
                    if num_open != n {
                        // Note clone does not keep the capacity, would be better to maintain that
                        cur_strs.0.push(cur_strs.0[i].clone());
                        let last_i = cur_strs.0.len() - 1;
                        cur_strs.0[last_i].push('(');
                        cur_strs.1.push((num_open + 1, num_close));
                    }
                    cur_strs.0[i].push(')');
                    cur_strs.1[i].1 += 1;
                } else if num_open != n {
                    cur_strs.0[i].push('(');
                    cur_strs.1[i].0 += 1;
                }
            }
        }
        cur_strs.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["()()", "(())"]);
        assert_eq!(Solution::generate_parenthesis(3), vec!["()()()", "(())()", "((()))", "()(())", "(()())"]);

        assert_eq!(2 + 2, 4);
    }
}
