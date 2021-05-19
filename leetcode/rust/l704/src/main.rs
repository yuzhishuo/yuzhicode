struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut 
        
        left = 0 as i32;
        let mut right = (nums.len() - 1) as i32;
        let mut mid = 0 as i32;
        while left <= right {
            mid = (left + right) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        -1
    }
}

fn main() {
    Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
}
