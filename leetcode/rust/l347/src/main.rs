use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ks = HashMap::new();
        for element in nums.iter() {
            let counter = ks.entry(*element).or_insert(0);
            *counter += 1;
        }
        let mut t = Vec::new();
        for element in ks.into_iter() {
            t.push((element.1, element.0));
        }

        t.sort_by(|a, b| b.cmp(a));
        let mut res = vec![];
        for index in 0..(k as usize) {
            res.push(t[index].1);
        }

        res
    }
}

fn main() {
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 1, 2, 2, 3, 4], 2),
        vec![1, 2]
    );
}
