struct Solution;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let size = ratings.len();
        if size < 2 {
            return size as i32;
        }
        let mut num = vec![1; size];
        for i in 1..size {
            if ratings[i] > ratings[i - 1] {
                num[i] = num[i - 1] + 1;
            }
        }
        for i in (1..size).rev() {
            if ratings[i] < ratings[i - 1] {
                num[i - 1] = num[i - 1].max(num[i] + 1);
            }
        }
        num.iter().sum::<usize>() as i32
    }
}
fn main() {
    assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
}
