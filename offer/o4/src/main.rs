/*
剑指 Offer 04. 二维数组中的查找
在一个 n * m 的二维数组中，每一行都按照从左到右递增的顺序排序，每一列都按照从上到下递增的顺序排序。请完成一个高效的函数，输入这样的一个二维数组和一个整数，判断数组中是否含有该整数。



示例:

现有矩阵 matrix 如下：

[
  [1,   4,  7, 11, 15],
  [2,   5,  8, 12, 19],
  [3,   6,  9, 16, 22],
  [10, 13, 14, 17, 24],
  [18, 21, 23, 26, 30]
]
给定 target = 5，返回 true。

给定 target = 20，返回 false。
*/

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

        false
    }
}

fn main() {
    println!("Hello, world!");
}
