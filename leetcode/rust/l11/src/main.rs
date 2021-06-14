struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut a = 0;
        for i in 0..height.len() - 1 {
            if height[i] <= a {
                continue;
            }
            for j in i + 1..height.len() {
                let tmp = height[j].min(height[i]) * (j - i) as i32;
                if tmp > ret {
                    ret = tmp;
                    a = height[j].min(height[i]);
                }
            }
        }
        ret
    }
}

fn main() {
    Solution::max_area(vec![1, 2, 3, 4, 5, 6]);
}
