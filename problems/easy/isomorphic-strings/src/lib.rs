pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map: [char; 128] = ['\0'; 128];
        let mut mapped: [bool; 128] = [false; 128];
        for pair in s.chars().zip(t.chars()) {
            if map[pair.0 as usize] != '\0' {
                if map[pair.0 as usize] != pair.1 {
                    return false;
                }
            } else {
                if mapped[pair.1 as usize] {
                    return false;
                }
                map[pair.0 as usize] = pair.1;
                mapped[pair.1 as usize] = true;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::is_isomorphic("egg".into(), "add".into()), true);
        assert_eq!(Solution::is_isomorphic("foo".into(), "bar".into()), false);
        assert_eq!(Solution::is_isomorphic("badc".into(), "bada".into()), false);
        assert_eq!(Solution::is_isomorphic("paper".into(), "title".into()), true);
    }
}
