struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x as u32;
        let mut ans: u32 = 0;
        //进制的本质
        let mut i: u32 = 32;
        loop {
            match i {
                0 => break,
                _defual => i -= 1,
            }
            ans <<= 1;
            ans += x & 1;
            x >>= 1;
        }
        return ans;
    }
}

fn main() {
    println!("Hello, world!");
}
