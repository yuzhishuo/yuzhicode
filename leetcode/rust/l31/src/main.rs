struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() == 1 {
            return;
        }
        let mut second = -1;
        for i in (1..nums.len()).rev() {
            if nums[i] > nums[i - 1] {
                second = (i - 1) as i32;
                break;
            }
        }

        if second == -1 {
            nums.sort();
            return;
        }

        let second = second as usize;
        for i in (second..nums.len()).rev() {
            if nums[second] < nums[i] {
                let t = nums[second];
                nums[second] = nums[i];
                nums[i] = t;
                break;
            }
        }

        let mut i = second + 1;
        let mut j = nums.len() - 1;

        while i < j {
            let t = nums[j];
            nums[j] = nums[i];
            nums[i] = t;
            i = i + 1;
            j = j - 1;
        }
        return;
    }
}

fn main() {
    Solution::next_permutation(vec![1, 2, 3]);
}
