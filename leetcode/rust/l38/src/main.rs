struct Solution;
impl Solution {
    #![allow(dead_code)]
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return String::from("1");
        }
        let mut v = vec![String::from("1")];
        for i in 1..=n as usize {
            let m = v[i - 1].clone();
            v.push(
                m.chars()
                    .fold(((1, 1, 'a'), "".to_string()), |res, c| {
                        let mut res = res;
                        if res.0 .0 == m.len() {
                            if res.0 .2 != 'a' {
                                if res.0 .2 == c {
                                    res.1.push_str(&(res.0 .1 + 1).to_string());
                                    res.1.push(c);
                                    return ((1, 1, 'a'), res.1);
                                } else {
                                    res.1.push_str(&(res.0 .1).to_string());
                                    res.1.push(res.0 .2);
                                }
                            }
                            res.1.push_str(&1.to_string());
                            res.1.push(c);
                            return ((1, 1, 'a'), res.1);
                        }

                        if res.0 .2 == 'a' {
                            return ((res.0 .0 + 1, 1, c), res.1);
                        } else if res.0 .2 == c {
                            return ((res.0 .0 + 1, res.0 .1 + 1, c), res.1);
                        } else {
                            res.1.push_str(&(res.0 .1).to_string());
                            res.1.push(res.0 .2);
                            return ((res.0 .0 + 1, 1, c), res.1);
                        }
                    })
                    .1,
            );
        }
        v[n as usize - 1].clone()
    }
}

fn main() {
    println!("Hello, world!");
}
