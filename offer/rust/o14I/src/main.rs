struct Solution;

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0 as usize; n+1];
        dp[2] = 1;

        for i in 3..n+1 {
            for j in 2..i {
                dp[i] = dp[i].max(j * (i - j)).max(j * dp[i - j]);
            }
        }
        dp[n] as i32
    }
}

fn main() {
    assert_eq!(Solution::cutting_rope(2), 1);
}
