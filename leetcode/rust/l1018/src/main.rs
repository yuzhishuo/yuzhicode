struct Solution;
impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        a.iter()
            .enumerate()
            .scan(0, move |sum, x| {
                *sum = ((*sum) *2 + x.1) % 5;
                Some(*sum  == 0)
            })
            .collect()
    }
}
fn main() {
    let _t = Solution::prefixes_div_by5(vec![0, 1, 1, 1, 1, 1]);
}