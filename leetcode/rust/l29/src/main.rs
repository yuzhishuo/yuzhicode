struct Solution;
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let mut a = dividend.abs();
        let b = divisor.abs();
        let mut res = 0;

        while a - b >= 0 {
            let mut x = 0;
            while a - (b << x << 1) >= 0 {
                x += 1;
            }

            res += 1 << x;
            a -= b << x;
        }

        if (dividend > 0) == (divisor > 0) {
            res
        } else {
            -res
        }
    }
}

fn main() {
    assert_eq!(Solution::divide(10,3),3);
}
