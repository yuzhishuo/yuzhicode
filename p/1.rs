use std::collections::HashSet;
use std::collections::BTreeSet;
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(self, a: Vec<i32>) -> Vec<i32> {
        
        let mut a = a;
        let mut result =BTreeSet::<i32>::new();

        for i in a.iter() {
            result.insert(*i);
        }
        a.clear();
        for i in result.iter() {
            a.push(*i);
        }

        a
     }


     pub fn remove_duplicates1(self, a: Vec<i32>) -> Vec<i32> {

        if a.len() < 1 { a }
        let mut a = a;

        let mut index = 0;

        let mut cur = a[0];
        println!("{:?}", cur);
        for i in 1..a.len() {

            if cur == a[i] {

            }

        }
        return a;
     }
}


fn main () {
    let t= Solution{};
    let t1 =  t.remove_duplicates1([1,2,3,4,5,].to_vec());

    println!("{:?}", t1);
}