struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let N = height.len();
        let mut stack = Vec::new();
        let mut ans = 0;

        for i in 0..N {
            while stack.len() > 0 && height[i] > height[stack[stack.len() - 1]] {
                let t = stack.pop().unwrap();

                if stack.len() == 0 {
                    break;
                }

                let l = stack[stack.len() - 1];
                let w = i - l - 1;
                let h = (height[i]).min(height[l]) - height[t];
                ans += (w as i32) * h;
            }

            stack.push(i);
        }
        ans
    }
}

fn main() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}
