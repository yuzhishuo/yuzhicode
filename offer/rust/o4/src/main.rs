struct Solution;
impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let col_len = matrix.len();
        if col_len == 0 || matrix[0].len() == 0 {
            return false;
        }
        let row_len = matrix[0].len();

        let (mut i, mut j) = (0, 0);

        loop {
            if matrix[i][j] == target {
                return true;
            }

            if i + 1 < col_len && matrix[i + 1][j] <= target {
                i = i + 1;
                continue;
            }
            if j + 1 < row_len && matrix[i][j + 1] <= target {
                j = j + 1;
                continue;
            }

            if j as i32 + 1 >= row_len as i32 {
                return false;
            }
            j = j + 1;
            loop {
                if i as i32 - 1 <= -1 {
                    return false;
                }

                i = i - 1;

                if matrix[i][j] <= target {
                    break;
                }
            }
        }
    }
}

fn main() {
    assert_eq!(
        Solution::find_number_in2_d_array(vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30]
        ],5),
        true
    );
}
