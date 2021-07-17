struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        (0..(1 << nums.len()))
            .map(|mask| {
                nums.iter()
                    .enumerate()
                    .filter_map(|(i, &v)| if mask & (1 << i) != 0 { Some(v) } else { None })
                    .collect()
            })
            .collect()
    }
}

fn main() {
    assert_eq!(
        Solution::subsets(vec![1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ]
    );
}
