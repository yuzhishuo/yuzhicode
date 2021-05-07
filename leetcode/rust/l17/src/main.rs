/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */

struct Solution;
// @lc code=start

impl Solution {
    fn layer_inner(result: &mut Vec<String>, s: &mut String) {
        let mut result = result;
        if s.is_empty() {
            return;
        }

        let first = s.pop().unwrap() as u32 - '0' as u32;
        const begin_line: u32 = 'a' as u32;
        let new_vec: &mut Vec<String> = &mut Vec::new();

        for elem in result.iter() {
            let current_point = begin_line + (first - 2) * 3 as u32;
            unsafe {
                let mut c = elem.clone();
                c.push(std::char::from_u32_unchecked(current_point));
                new_vec.push(c);
                let mut c = elem.clone();
                c.push(std::char::from_u32_unchecked(current_point + 1));
                new_vec.push(c);
                let mut c = elem.clone();
                c.push(std::char::from_u32_unchecked(current_point + 2));
                new_vec.push(c);
            }
        }
        result.clear();
        for elem in new_vec.iter() {
            result.push(elem.clone());
        }

        Self::layer_inner(result, s);
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mut s = "".to_string();
        let mut digits = digits;


        while !digits.is_empty() {

            s.push(digits.pop().unwrap());
        }



        let mut new_vec = Vec::<String>::new();
        let mut digits = s;

        let first = digits.pop().unwrap() as u32 - '0' as u32;
        const begin_line: u32 = 'a' as u32;

        unsafe {
            let current_point = begin_line + (first - 2) * 3 as u32;
            let mut c = "".to_string();
            c.push(std::char::from_u32_unchecked(current_point));
            new_vec.push(c);
            let mut c = "".to_string();
            c.push(std::char::from_u32_unchecked(current_point + 1));
            new_vec.push(c);
            let mut c = "".to_string();
            c.push(std::char::from_u32_unchecked(current_point + 2));
            new_vec.push(c);
        }
        Self::layer_inner(&mut new_vec, &mut digits.clone());
        new_vec
    }
}
// @lc code=end

fn main() {
    println!("Hello, world!");
}
