struct Solution;
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.push(new_interval);
        intervals.sort_unstable();
        let mut answer = Vec::new();
        for x in intervals {
            match answer.last_mut() {
                None => answer.push(x),
                Some(last) => {
                    if x[0] > last[1] {
                        answer.push(x);
                    } else {
                        last[1] = last[1].max(x[1]);
                    }
                }
            }
        }
        answer
    }
}

fn main() {
    assert_eq!(
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        vec![[1, 5], [6, 9]]
    );
}
