struct Solution;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable();
        let mut v: Vec<Vec<i32>> = vec![];

        for x in intervals.iter() {
            if let Some(c) = v.last_mut() {
                if c[0] > x[0] && c[1] < x[1] {
                    c[0] = x[0];
                    c[1] = x[1];
                } else if c[1] >= x[0] {
                    c[1] = if c[1] > x[1] { c[1] } else { x[1] }
                } else {
                    v.push(x.clone());
                }
            } else {
                v.push(x.clone());
            }
        }
        v
    }
}
fn main() {
    assert_eq!(
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
}
