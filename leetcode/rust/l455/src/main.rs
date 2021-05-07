struct Solution;

impl Solution {
    pub fn find_content_children1(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let (mut g, mut s) = (g, s);
        g.sort();
        s.sort();
        let mut index = 0;
        let mut res = 0;
        for elem in g.iter() {
            if index >= s.len() {
                return res;
            }
            for i in index..s.len() {
                if s[i] - elem >= 0 {
                    index = i + 1;
                    res = res + 1;
                    break;
                }
            }
        }
        res
    }
}
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let (mut g, mut s) = (g, s);
        g.sort();
        s.sort();
        let (mut child, mut cookie) = (0, 0);

        while child < g.len() && cookie < s.len() {
            if g[child] <= s[cookie] {
                child = child + 1;
            }
            cookie = cookie + 1;
        }
        child as i32
    }
}
fn main() {}
