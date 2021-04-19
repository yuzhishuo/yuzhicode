/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 */




// @lc code=start

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 { return nums.len() as i32; }
        
        let mut hs = HashSet::<i32>::new();

        for cur in nums.iter() {
            hs.insert(*cur);
        }

        let mut res = 1;
        let mut new_res = 1;
        
        let mut new_cur = 0;
        for cur in hs.iter() {

            if hs.contains(&(*cur-1)) {
                new_res += 1;
                new_cur = *cur +1;
                while hs.contains(&new_cur) {
                    new_res += 1;
                    new_cur = new_cur +1;
                }
                res = if res > new_res { res } else { new_res };
            }
            new_res = 1;
        }
        if res > new_res {res} else { new_res }
    }
}
// @lc code=end


use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 { return nums.len() as i32; }
        let mut hp = BinaryHeap::<i32>::new();

        for cur in nums.iter() {
           
            hp.push(*cur);
        }

        let mut res = 1;
        let mut cur = 1;
        let mut prev = 0;
        while !hp.is_empty() {

            let new_val = hp.pop();
            match new_val.unwrap() - prev {

                -1  =>  { cur += 1;},
                0 => {},
                _ => {
                    res = if res > cur {res} else { cur };
                    cur = 1;
                }
            }
            prev = new_val.unwrap();
        }

        if res > cur {res} else { cur }
    }
}