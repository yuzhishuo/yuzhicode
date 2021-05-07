use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for itr in nums.iter() {
            if !set.insert(*itr) {
                return *itr;
            }
        }

        0
    }
}

fn main() {
    println!("Hello, world!");
}
