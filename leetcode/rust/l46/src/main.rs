struct Solution;
impl Solution {
    #![allow(dead_code)]
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(
            res: &mut Vec<Vec<i32>>,
            nums: &[i32],
            combination: &mut Vec<i32>,
            used: &mut Vec<bool>,
            index: usize,
            len: usize,
        ) {
            if index >= len {
                res.push(combination.to_vec());
                return;
            }
            for i in 0..len {
                if !used[i] {
                    combination.push(nums[i]);
                    used[i] = true;
                    helper(res, nums, combination, used, index + 1, len);
                    combination.pop();
                    used[i] = false;
                }
            }
        }
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut combination: Vec<i32> = Vec::new();
        let len = nums.len();
        let mut used: Vec<bool> = vec![false; len];
        helper(&mut res, &nums, &mut combination, &mut used, 0, len);
        res
    }
}

fn main() {
    println!("no test!");
}
