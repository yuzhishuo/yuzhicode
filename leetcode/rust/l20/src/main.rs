struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let mut ret = true;
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                }
                ')' | ']' | '}' => {
                    if let Some(ch) = stack.pop() {
                        if ch == '(' && c == ')' || ch == '[' && c == ']' || ch == '{' && c == '}' {
                            continue;
                        } else {
                            ret = false;
                            break;
                        }
                    } else {
                        ret = false;
                        break;
                    }
                }
                _ => {}
            }
        }
        if stack.len() > 0 {
            ret = false;
        }
        ret
    }
}

fn main() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
}
