struct Solution;

impl Solution {
    pub fn sum_nums(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        return n + Self::sum_nums(n - 1);
    }

    pub fn sum_nums1(n: i32) -> i32 {
        let mut res: i32 = 0;
        let mut n = n;
        loop {
            match n {
                0 => return res,
                _defual => {
                    res += n;
                    n = n - 1;
                }
            }
        }
    }

    pub fn sum_nums2(n: i32) -> i32 {
        let mut res: i32 = 0;
        let mut n = n;
        while n != 0 {
            res += n;
            n = n - 1;
        }

        res
    }
}

fn main() {
    println!("Hello, world!");
}
