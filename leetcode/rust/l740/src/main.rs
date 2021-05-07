struct Solution;

impl Solution {
    pub fn rob(ref mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return nums[0];
        }
        let mut first = nums[0];
        let mut second = nums[0].max(nums[1]);

        for i in 2..len {
            let temp = second;
            second = second.max(first + nums[i]);
            first = temp;
        }

        second
    }

    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut ans = 0;
        let mut sum = vec![nums[0]];
        for i in 1..nums.len() {
            let val = nums[i];
            if val == nums[i - 1] {
                let t = sum.len() - 1;
                let old_val = sum[t];
                sum[t] = old_val + val;
            } else if val == nums[i - 1] + 1 {
                sum.push(val);
            } else {
                ans += Self::rob(sum);
                sum = vec![val];
            }
        }
        ans += Self::rob(sum);
        ans
    }
}

fn main() {
    println!("{}", Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]));
}
