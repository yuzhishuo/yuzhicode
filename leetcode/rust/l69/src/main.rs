
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

    pub fn my_sqrt1(x: i32) -> i32 {
        let mut r = x;
        let mut l = 0;

        let mut ans = -1;
        while l < r {
            let mid = (l + r) / 2;
            if mid * mid <= x {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        ans
    }
}

fn main() {
    assert_eq!(Solution::my_sqrt(9), 3);
    assert_eq!(Solution::my_sqrt1(10), 3);
