struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(100);
        let mut carry = -1;
        s.bytes().enumerate().fold(0, |s, (i, x)| {
            if x == b'(' {
                stack.push(carry);
                carry = i as i32;
                s
            } else if let Some(z) = stack.pop() {
                carry = z;
                s.max(i as i32 - z)
            } else {
                carry = i as i32;
                s
            }
        })
    }
}

fn main() {
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
}
