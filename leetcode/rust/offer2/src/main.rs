struct Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        s.replace(" ", "%20")
    }
}

// 执行用时：
// 0 ms
// , 在所有 Rust 提交中击败了
// 100.00%
// 的用户
// 内存消耗：
// 1.9 MB
// , 在所有 Rust 提交中击败了
// 94.55%
// 的用户

// !!!  手动替换的优势出现了

impl Solution {
    pub fn replace_space1(s: String) -> String {
        let mut res = String::new();

        for (_, c) in s.char_indices() {
            if c.is_whitespace() {
                res += "%20";
                continue;
            }
            res += &c.to_string();
        }
        res
    }
}

impl Solution {
    pub fn replace_space2(s: String) -> String {
        s.chars()
            .map(|x| {
                (if x.is_whitespace() {
                    vec!['%', '2', '0'].into_iter()
                } else {
                    vec![x].into_iter()
                })
            })
            .flatten()
            .collect()
    }
}

impl Solution {
    pub fn replace_space3(s: String) -> String {
        let mut insert = vec!['%', '2', '0'];
        let mut res: Vec<char> = Vec::new();
        for c in s.chars() {
            if c != ' ' {
                res.push(c);
            } else {
                res.append(&mut insert.clone());
            }
        }

        res.into_iter().collect()
    }
}

impl Solution {
    pub fn replace_space4(s: String) -> String {
        // Passed 0ms 2mb
        s.chars()
            .map(|ch| {
                if !ch.is_ascii_whitespace() {
                    ch.to_string()
                } else {
                    "%20".to_string()
                }
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
