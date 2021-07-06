use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 哈希集合，记录每个字符是否出现过
        let mut occ: HashSet<char> = HashSet::new();
        let cv: Vec<char> = s.chars().collect();
        let n = s.len();
        // 右指针，初始值为 0，相当于我们在字符串的左边界的左侧，还没有开始移动
        let mut rk = 0;
        let mut ans = 0;
        for i in 0..n {
            if i != 0 {
                // 左指针向右移动一格，移除一个字符
                occ.remove(&cv[i - 1]);
            }
            while rk < n && !occ.contains(&cv[rk]) {
                // 不断地移动右指针
                occ.insert(cv[rk]);
                rk += 1;
            }
            // 第 i 到 rk 个字符是一个极长的无重复字符子串
            ans = ans.max(rk - i);
        }
        ans as i32
    }
}

fn main() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
}
