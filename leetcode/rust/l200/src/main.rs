struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut union_find = UnionFind::new(&grid);
        let (row, col) = (grid.len(), grid[0].len());
        for r in 0..row {
            for c in 0..col {
                if grid[r][c] == '0' {
                    continue;
                }
                if r >= 1 {
                    union_find.union(r * col + c, (r - 1) * col + c);
                }
                if r + 1 < row {
                    union_find.union(r * col + c, (r + 1) * col + c);
                }
                if c >= 1 {
                    union_find.union(r * col + c, r * col + c - 1);
                }
                if c + 1 < col {
                    union_find.union(r * col + c, r * col + c + 1);
                }
            }
        }
        union_find.count
    }
}

struct UnionFind {
    count: i32,
    parent: Vec<i32>,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(grid: &Vec<Vec<char>>) -> Self {
        let (row, col) = (grid.len(), grid[0].len());
        let (mut parent, mut count) = (vec![-1 as i32; row * col], 0);
        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == '1' {
                    parent[i * col + j] = (i * col + j) as i32;
                    count += 1;
                }
            }
        }
        UnionFind {
            count: count,
            parent: parent,
            rank: vec![0; row * col],
        }
    }

    fn find(&mut self, idx: usize) -> usize {
        if self.parent[idx] != idx as i32 {
            self.parent[idx] = self.find(self.parent[idx] as usize) as i32
        }
        return self.parent[idx] as usize;
    }

    fn union(&mut self, x_index: usize, y_index: usize) {
        if self.parent[x_index] == -1 || self.parent[y_index] == -1 {
            return;
        }
        let x_parent = self.find(x_index);
        let y_parent = self.find(y_index);
        if x_parent != y_parent {
            if self.rank[x_parent] > self.rank[y_parent] {
                self.parent[y_parent] = x_parent as i32;
            } else if self.rank[x_parent] < self.rank[y_parent] {
                self.parent[x_parent] = y_parent as i32;
            } else {
                self.parent[y_parent] = x_parent as i32;
                self.rank[x_parent] += 1;
            }
            self.count -= 1;
        }
    }
}

fn main() {
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ]),
        1
    );
}
