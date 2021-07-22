struct Solution;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut i = 0 as usize;

        while i < nums.len() {
            if nums[i] == i as i32 {
                i = i + 1;
                continue;
            }
            if nums[nums[i] as usize] == nums[i] {
                return nums[i];
            }

            let t = nums[i] as usize;
            nums[t] = nums[i];
            nums[i] = t as i32;
        }
        1
    }
}

fn main() {
    assert_eq!(
        Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]) == 2
            || Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]) == 3,
        true
    )
}
