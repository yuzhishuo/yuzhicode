struct Solution;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut res = std::i32::MAX - 3000;
        for (p, cur) in (0..).zip(nums.iter()) {
            let mut i = p + 1;
            let mut j = nums.len() - 1;

            while i < j {
                let sum = nums[i] + nums[j] + cur;
                if sum == target {
                    return sum;
                }
                if (sum - target).abs() < (res - target).abs() {
                    res = sum;
                }
                if sum > target {
                    let mut k0 = j - 1;
                    while i < k0 && nums[k0] == nums[j] {
                        k0 = k0 - 1;
                    }
                    j = k0;
                } else {
                    let mut j0 = i + 1;
                    while j0 < j && nums[j0] == nums[i] {
                        j0 = j0 + 1;
                    }
                    i = j0;
                }
            }
        }
        res
    }
}

fn main() {
    assert_eq!(Solution::three_sum_closest(vec![-3, -2, -5, 3, -4], -1), -2);
}
