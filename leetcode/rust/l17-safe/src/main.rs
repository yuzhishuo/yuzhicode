use std::{collections::HashMap, vec};
struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = vec![];
        if digits.len() == 0 {
            return res;
        }

        let mut map = HashMap::new();
        map.insert('2', vec!['a', 'b', 'c']);
        map.insert('3', vec!['d', 'e', 'f']);
        map.insert('4', vec!['g', 'h', 'i']);
        map.insert('5', vec!['j', 'k', 'l']);
        map.insert('6', vec!['m', 'n', 'o']);
        map.insert('7', vec!['p', 'q', 'r', 's']);
        map.insert('8', vec!['t', 'u', 'v']);
        map.insert('9', vec!['w', 'x', 'y', 'z']);

        Solution::backtrack(
            &mut res,
            &map,
            &digits.chars().collect(),
            0,
            &mut String::new(),
        );
        res
    }

    fn backtrack(
        combinations: &mut Vec<String>,
        map: &HashMap<char, Vec<char>>,
        digits: &Vec<char>,
        index: usize,
        combination: &mut String,
    ) {
        if index == digits.len() {
            combinations.push(combination.clone());
        } else {
            let digit = digits[index];
            let letters = map.get(&digit).unwrap();
            for i in 0..letters.len() {
                combination.push(letters[i]);
                Solution::backtrack(combinations, map, digits, index + 1, combination);
                combination.remove(index);
            }
        }
    }
}

fn main() {
    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        vec![
            "ad".to_string(),
            "ae".to_string(),
            "af".to_string(),
            "bd".to_string(),
            "be".to_string(),
            "bf".to_string(),
            "cd".to_string(),
            "ce".to_string(),
            "cf".to_string()
        ]
    );
}
