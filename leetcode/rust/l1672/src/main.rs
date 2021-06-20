struct Solution;
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|x| x.iter().sum()).max().unwrap()
    }
}

fn main() {
    assert_eq!(
        Solution::maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
        17
    );
}
