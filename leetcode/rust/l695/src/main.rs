use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let m_i32 = m as i32;
        let n_i32 = n as i32;

        let mut queue = VecDeque::new() as VecDeque<(i32, i32)>;

        let mut max_count = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 1 {
                    continue;
                }
                queue.push_back((i as i32, j as i32));
                let mut curr_count = 0;
                while !queue.is_empty() {
                    let (curr_x, curr_y) = queue.pop_front().unwrap();
                    if grid[curr_x as usize][curr_y as usize] == 2 {
                        continue;
                    }
                    grid[curr_x as usize][curr_y as usize] = 2;
                    curr_count += 1;
                    for &(dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        // 注意不能用usize，因为下一对x,y可能是负数
                        let (x, y) = (curr_x + dx, curr_y + dy);
                        if x < 0 || x >= m_i32 || y < 0 || y >= n_i32 {
                            continue;
                        }
                        if grid[x as usize][y as usize] != 1 {
                            continue;
                        }
                        queue.push_back((x, y));
                    }
                    max_count = max_count.max(curr_count);
                }
            }
        }
        max_count
    }
}
fn main() {
    assert_eq!(
        Solution::max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ]),
        6
    );
}
