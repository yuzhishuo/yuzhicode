struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for cur in nums.iter() {
            res = res ^ cur;
        }
        res
    }
}

fn main() {
    assert_eq!(Solution::single_number(vec![1,2,2]), 1);
}
