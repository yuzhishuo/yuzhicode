struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut current = 0i32;

        for itr in nums.iter() {
            if current != *itr {
                return current;
            } else {
                current += 1;
                continue;
            }
        }
        current
    }
}

fn main() {
    println!("Hello, world!");
}
