struct Solution {}

impl Solution {
    const ITEMS: [[i32; 2]; 4] = [
        [-1, 0],
        [0, -1],
        [1, 0],
        [0, 1],
    ];
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid_mut: Vec<Vec<i32>> = grid;
        let mut ans = 0;
        let m = grid_mut.len();
        let n = grid_mut[0].len();
        for i in 0..m {
            for j in vec![0, n - 1] {
                let num = grid_mut[i][j];
                if num == 1 {
                    grid_mut[i][j] = 2;
                    Solution::solve(i, j, &mut grid_mut, m as i32, n as i32);
                }
            }
        }
        for i in vec![0, m - 1] {
            for j in 0..n {
                let num = grid_mut[i][j];
                if num == 1 {
                    grid_mut[i][j] = 2;
                    Solution::solve(i, j, &mut grid_mut, m as i32, n as i32);
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                let num = grid_mut[i][j];
                if num == 1 {
                    ans += 1;
                }
            }
        }
        return ans;
    }
    pub fn solve(i: usize, j: usize, grid: &mut Vec<Vec<i32>>, m: i32, n: i32) {
        // 往四边走
        let i_i32 = i as i32;
        let j_i32 = j as i32;
        for item in Solution::ITEMS {
            let next_i_i32: i32 = i_i32 + item[0];
            let next_j_i32: i32 = j_i32 + item[1];
            if next_i_i32 >= 0 && next_i_i32 < m && next_j_i32 >= 0 && next_j_i32 < n {
                let next_i: usize = next_i_i32 as usize;
                let next_j: usize = next_j_i32 as usize;
                if grid[next_i][next_j] == 1 {
                    grid[next_i][next_j] = 2;
                    Solution::solve(next_i, next_j, grid, m, n);
                }
            }
        }
    }
}