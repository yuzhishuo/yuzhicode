struct Solution;
impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![999; n + 1];
        dp[0] = 0;
        for j in (1..n + 1) {
            for k in 1..j + 1 {
                dp[j] = (k.max(dp[j - k] + 1)).min(dp[j]);
            }
        }
        dp[n] as i32
    }
}
fn main() {
    assert_eq!(Solution::two_egg_drop(2), 2);
}
