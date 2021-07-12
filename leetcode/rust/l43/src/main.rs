struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let m = num1.len();
        let n = num2.len();
        let num1: Vec<char> = num1.chars().collect();
        let num2: Vec<char> = num2.chars().collect();

        let mut pos = vec![0; m + n];

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let mul = (num1[i] as u8 - 48) * (num2[j] as u8 - 48);
                let p1 = i + j;
                let p2 = i + j + 1;
                let sum = mul + pos[p2];

                pos[p1] += sum / 10;
                pos[p2] = (sum) % 10;
            }
        }

        let mut res = String::new();
        for i in 0..pos.len() {
            if !(res.len() == 0 && pos[i] == 0) {
                res.push((pos[i] + 48) as char);
            }
        }

        if res.len() == 0 {
            "0".to_string()
        } else {
            res
        }
    }
}

fn main() {
    assert_eq!(
        Solution::multiply("2".to_string(), "2".to_string()),
        "4".to_string()
    );
}
