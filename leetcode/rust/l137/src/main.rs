use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in nums {
            if let Some(x) = map.get_mut(&n) {
                *x = *x + 1;
            } else {
                map.insert(n, 1);
            }
        }
        for (key, val) in map.iter() {
            if *val == 1 {
                return *key;
            }
        }
        -1
    }
    // https://leetcode-cn.com/problems/single-number-ii/solution/jian-dan-ban-wei-yun-suan-rong-yi-li-jie-y01o/
    pub fn single_number1(nums: Vec<i32>) -> i32 {
        let mut t0 = -1;
        let mut t1 = 0;
        let mut t2 = 0;
        for i in nums {
            let v0 = t0 & i;
            let v1 = t1 & i;
            let v2 = t2 & i;

            t0 = (t0 ^ v0) | v2;
            t1 = (t1 ^ v1) | v0;
            t2 = (t2 ^ v2) | v1;
        }
        t1
    }
}

fn main() {
    assert_eq!(Solution::single_number(vec![2, 2, 3, 2], 3));
    assert_eq!(Solution::single_number1(vec![2, 2, 3, 2], 3));
}
