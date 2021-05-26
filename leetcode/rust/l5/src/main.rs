struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut res = String::new();
        let s: Vec<char> = s.chars().collect();
        let mut max_len = 0;
        for i in (0..n).rev() {
            for j in i..n {
                if s[i] == s[j] && (j - i < 2 || dp[i + 1][j - 1]) {
                    dp[i][j] = true;

                    if j - i + 1 > max_len {
                        res = s[i..j + 1].iter().collect();
                        max_len = j - i + 1;
                    }
                }
            }
        }

        res
    }
}

fn main() {
    println!("Hello, world!");
}
