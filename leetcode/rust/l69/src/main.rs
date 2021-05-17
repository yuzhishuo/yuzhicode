struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as f64;
        let mut k = 1.0f64;
        while (k * k - x).abs() > 0.00000001 {
            k = (k + x / k) / 2.0;
        }
        k as i32
    }
}

fn main() {
    assert_eq!(Solution::my_sqrt(9), 3);
}
