use std::borrow::{Borrow, BorrowMut};
struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut v = vec![];
        dfs(
            0_usize,
            s.borrow(),
            0,
            String::new().borrow(),
            v.borrow_mut(),
        );
        v
    }
}
pub fn dfs(start: usize, s: &str, segment: usize, strs: &str, list: &mut Vec<String>) {
    for i in 0..3 {
        let mut strs = strs.to_string();
        let end = start + i;
        if end < s.len() && segment <= 3 {
            strs += s[start..=end].to_string().as_ref();
            if s[start..=end].parse::<i32>().unwrap() > 255 {
                continue;
            }
            if s.as_bytes()[start] == 48 && end - start >= 1 {
                continue;
            }
            if end == s.len() - 1 && segment == 3 {
                list.push(strs.parse().unwrap());
            } else {
                strs.push(".".parse().unwrap());
                dfs(end + 1, s, segment + 1, strs.borrow(), list);
            }
        }
    }
}
fn main() {
    Solution::restore_ip_addresses(String::from("125125125125"));
}
