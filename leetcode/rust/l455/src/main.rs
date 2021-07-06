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
    pub fn find_content_children2(g: Vec<i32>, s: Vec<i32>) -> i32 {
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

impl Solution {
    pub fn find_content_children3(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let (mut vg, mut vs) = (g.iter(), s.iter());
        let (mut g, mut s) = (vg.next(), vs.next());
        let mut ans = 0;
        while g.is_some() && s.is_some() {
            if g <= s {
                ans += 1;
                g = vg.next();
            }
            s = vs.next();
        }
        return ans;
    }
}
fn main() {
    assert_eq!(
        Solution::find_content_children1(vec![1, 2, 3], vec![1, 1]),
        1
    );
    assert_eq!(
        Solution::find_content_children2(vec![1, 2, 3], vec![1, 1]),
        1
    );
    assert_eq!(
        Solution::find_content_children3(vec![1, 2, 3], vec![1, 1]),
        1
    );
}
