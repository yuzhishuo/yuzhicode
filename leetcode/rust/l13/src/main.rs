struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.bytes()
            .rev()
            .fold((0, 0), |res, cur| {
                let n: i32 = match cur {
                    b'I' => 1,
                    b'V' => 5,
                    b'X' => 10,
                    b'L' => 50,
                    b'C' => 100,
                    b'D' => 500,
                    b'M' => 1000,
                    _ => -9999,
                };
                (if n < res.1 { res.0 - n } else { res.0 + n }, n)
            })
            .0
    }
}

fn main() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
}
