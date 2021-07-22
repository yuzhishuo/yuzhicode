struct Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        s.chars().fold("".to_string(), |mut res, ch| {
            if ch == ' ' {
                return res + "%20";
            }
            res.push(ch);
            res
        })
    }
}

fn main() {
    assert_eq!(
        Solution::replace_space("We are happy.".to_string()),
        "We%20are%20happy."
    );
}
