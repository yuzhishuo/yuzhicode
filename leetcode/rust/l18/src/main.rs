struct Solution;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();
        let len = nums.len();

        let mut res = vec![];
        for i in 0..len - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] + nums[i + 1] + nums[i + 2] + nums[i + 3] > target {
                break;
            }
            if nums[i] + nums[len - 3] + nums[len - 2] + nums[len - 1] < target {
                continue;
            }

            for j in (i + 1)..len - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                if nums[i] + nums[j] + nums[j + 1] + nums[j + 2] > target {
                    break;
                }
                if nums[i] + nums[j] + nums[len - 2] + nums[len - 1] < target {
                    continue;
                }
                let mut m = j + 1;
                let mut n = len - 1;
                while m < n {
                    if nums[i] + nums[j] + nums[m] + nums[n] == target {
                        res.push(vec![nums[i], nums[j], nums[m], nums[n]]);

                        while (m < n && nums[m] == nums[m + 1]) {
                            m = m + 1;
                        }
                        m = m + 1;
                        while (m < n && nums[n] == nums[n - 1]) {
                            n = n - 1;
                        }
                        n = n - 1;
                    } else if nums[i] + nums[j] + nums[m] + nums[n] < target {
                        m = m + 1;
                    } else {
                        n = n - 1;
                    }
                }
            }
        }
        res
    }
}
fn main() {
    Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
}
