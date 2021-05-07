struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;

        for i in nums.iter() {
            if target == *i {
                res += 1;
            } else {
                res;
            };
        }

        res
    }
}

fn main() {
    println!("Hello, world!");
}
