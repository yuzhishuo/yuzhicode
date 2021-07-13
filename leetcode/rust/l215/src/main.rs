struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        // nums.shuffle();
        let mut l = 0;
        let mut r = nums.len() - 1;
        let target = nums.len() - k as usize;
        while l < r {
            let mid = Self::quick_selection(&mut nums, l as i32, r as i32) as usize;
            if mid == target {
                return nums[mid];
            }
            if mid < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        return nums[l];
    }

    fn quick_selection(nums: &mut Vec<i32>, l: i32, r: i32) -> i32 {
        let mut i = l + 1;
        let mut j = r;

        loop {
            while i < r && nums[i as usize] <= nums[l as usize] {
                i = i + 1;
            }
            while l < j && nums[j as usize] >= nums[l as usize] {
                j = j - 1;
            }

            if i >= j {
                break;
            }
            nums.swap(i as usize, j as usize);
        }
        nums.swap(l as usize, j as usize);
        j
    }
}

fn main() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5, 6], 1), 6);
}
