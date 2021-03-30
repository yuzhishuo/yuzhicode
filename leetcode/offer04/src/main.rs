struct Solution;

impl Solution {
    pub fn find_number_in2_d_array1(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for com in matrix.iter() {
            for i in com.iter() {
                if *i == target {
                    return true;
                }
            }
        }

        false
    }
}

impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for i in 0..matrix.len() {
            for j in 0..matrix.get(i).unwrap().len() {
                if *matrix.get(i).unwrap().get(j).unwrap() == target {
                    return true;
                }

                if *matrix.get(i).unwrap().get(j).unwrap() > target {
                    continue;
                }
            }
        }
        false
    }
}

impl Solution {
    pub fn find_number_in2_d_array3(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (
            matrix.len(),
            if matrix.len() > 1 { matrix[0].len() } else { 0 },
        );

        let mut i = 0;
        let mut j = m - 1;

        loop {
            if i == j {
                break;
            }

            let mut mid = (i + j) / 2 as usize;

            match matrix[mid][0].cmp(&target) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => j = mid + 1,
                std::cmp::Ordering::Greater => i = mid - 1,
            }
        }
        true
    }
}

/// 二分查找
fn main() {
    let v = vec![1, 2, 3, 5, 9, 10, 11, 12];

    let mut i = 0;
    let mut j = v.len() - 1;

    'ff: loop {
        if i > j {
            break 'ff;
        }
        let mut mid = (i + j) / 2 as usize;
        match v[mid].cmp(&4) {
            std::cmp::Ordering::Equal => {
                println!("find {}", 4);
                return;
            }
            std::cmp::Ordering::Less => i = mid + 1,
            std::cmp::Ordering::Greater => j = mid - 1,
        }
    }

    println!("{}", i);
}
