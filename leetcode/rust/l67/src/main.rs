struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = if a.len() > b.len() { (a, b) } else { (b, a) };

        if a == "0" && b == "0" {
            return "0".to_string();
        }

        #[allow(unused_parens)]
        b.chars().rev().enumerate().fold(
            a,
            (|res, x| {
                if x.1 == '0' {
                    return res;
                }
                let mut result = res.clone().into_bytes();
                for cur in (0..result.len() - x.0).rev() {
                    if result[cur] == '0' as u8 {
                        result[cur] = '1' as u8;
                        return String::from_utf8(result).unwrap();
                    }

                    result[cur] = '0' as u8;
                }

                if result[0] == '0' as u8 {
                    result.insert(0, '1' as u8);
                }

                return String::from_utf8(result).unwrap();
            }),
        )
    }
}

fn main() {
    assert_eq!(
        Solution::add_binary("1".to_string(), "1".to_string()),
        "10".to_string()
    )
}
