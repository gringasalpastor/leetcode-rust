pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 5,
            5 => 8,
            6 => 13,
            7 => 21,
            8 => 34,
            9 => 55,
            10 => 89,
            11 => 144,
            12 => 233,
            13 => 377,
            14 => 610,
            15 => 987,
            16 => 1597,
            17 => 2584,
            18 => 4181,
            19 => 6765,
            20 => 10946,
            21 => 17711,
            22 => 28657,
            23 => 46368,
            24 => 75025,
            25 => 121393,
            26 => 196418,
            27 => 317811,
            28 => 514229,
            29 => 832040,
            30 => 1346269,
            31 => 2178309,
            32 => 3524578,
            33 => 5702887,
            34 => 9227465,
            35 => 14930352,
            36 => 24157817,
            37 => 39088169,
            38 => 63245986,
            39 => 102334155,
            40 => 165580141,
            41 => 267914296,
            42 => 433494437,
            43 => 701408733,
            44 => 1134903170,
            45 => 1836311903,
            _ => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climb_stairs() {
        assert_eq!(Solution::climb_stairs(0), 0);
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
        assert_eq!(Solution::climb_stairs(6), 13);
        assert_eq!(Solution::climb_stairs(7), 21);
        assert_eq!(Solution::climb_stairs(8), 34);
        assert_eq!(Solution::climb_stairs(9), 55);
        assert_eq!(Solution::climb_stairs(10), 89);
        assert_eq!(Solution::climb_stairs(11), 144);
        assert_eq!(Solution::climb_stairs(12), 233);
        assert_eq!(Solution::climb_stairs(13), 377);
        assert_eq!(Solution::climb_stairs(14), 610);
        assert_eq!(Solution::climb_stairs(15), 987);
        assert_eq!(Solution::climb_stairs(16), 1597);
        assert_eq!(Solution::climb_stairs(17), 2584);
        assert_eq!(Solution::climb_stairs(18), 4181);
        assert_eq!(Solution::climb_stairs(19), 6765);
        assert_eq!(Solution::climb_stairs(20), 10946);
        assert_eq!(Solution::climb_stairs(21), 17711);
        assert_eq!(Solution::climb_stairs(22), 28657);
        assert_eq!(Solution::climb_stairs(23), 46368);
        assert_eq!(Solution::climb_stairs(24), 75025);
        assert_eq!(Solution::climb_stairs(25), 121393);
        assert_eq!(Solution::climb_stairs(26), 196418);
        assert_eq!(Solution::climb_stairs(27), 317811);
        assert_eq!(Solution::climb_stairs(28), 514229);
        assert_eq!(Solution::climb_stairs(29), 832040);
        assert_eq!(Solution::climb_stairs(30), 1346269);
        assert_eq!(Solution::climb_stairs(31), 2178309);
        assert_eq!(Solution::climb_stairs(32), 3524578);
        assert_eq!(Solution::climb_stairs(33), 5702887);
        assert_eq!(Solution::climb_stairs(34), 9227465);
        assert_eq!(Solution::climb_stairs(35), 14930352);
        assert_eq!(Solution::climb_stairs(36), 24157817);
        assert_eq!(Solution::climb_stairs(37), 39088169);
        assert_eq!(Solution::climb_stairs(38), 63245986);
        assert_eq!(Solution::climb_stairs(39), 102334155);
        assert_eq!(Solution::climb_stairs(40), 165580141);
        assert_eq!(Solution::climb_stairs(41), 267914296);
        assert_eq!(Solution::climb_stairs(42), 433494437);
        assert_eq!(Solution::climb_stairs(43), 701408733);
        assert_eq!(Solution::climb_stairs(44), 1134903170);
        assert_eq!(Solution::climb_stairs(45), 1836311903);
    }
}
