struct Solution;

impl Solution {
    pub fn is_ok(question: Vec<i32>, sum: i32) -> i32 {
        let mut question = question;

        for mut cur in question.iter() {
            cur = &(*cur % sum);
        }
        1
    }
}

fn main() {
    println!("Hello, world!");
}