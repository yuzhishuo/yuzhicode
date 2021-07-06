struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut start = 0;
        let mut end = nums.len() - 1;
        while start <= end {
            let mid = (start + end) / 2;
            if nums[mid] == target {
                return true;
            }
            if nums[start] == nums[mid] {
                // 无法判断哪个区间是增序的
                start = start + 1;
            } else if nums[mid] <= nums[end] {
                // 右区间是增序的
                if target > nums[mid] && target <= nums[end] {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            } else {
                // 左区间是增序的
                if target >= nums[start] && target < nums[mid] {
                    end = mid - 1;
                } else {
                    start = mid + 1;
                }
            }
        }
        return false;
    }
}

fn main() {
    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 5), true);
}
