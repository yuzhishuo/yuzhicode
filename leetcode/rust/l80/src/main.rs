/*
 * @lc app=leetcode.cn id=80 lang=rust
 *
 * [80] 删除有序数组中的重复项 II
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

        if nums.len() <= 2 { return nums.len() as i32; }


        let mut index = 2;
        for i in 2..nums.len() {

            if nums[i] != nums[index-2] {
                nums[index] = nums[i];
                index +=1;
            }
           
        }
        index as i32
    }
}
// @lc code=end



fn main() {
    println!("Hello, world!");
}
