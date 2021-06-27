struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let table1 = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let table2 = vec![
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let mut s = num;
        let mut rtn = "".to_string();
        for i in 0..13 {
            let count = s / table1[i];
            s = s % table1[i];
            for _ in 0..count {
                rtn.push_str(table2[i as usize]);
            }
        }
        rtn
    }
}

fn main() {
    assert_eq!(Solution::int_to_roman(3), "III".to_string());
}
