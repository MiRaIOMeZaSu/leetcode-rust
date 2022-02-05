struct Solution {}

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        // 深度优先遍历法,如何保证引用和改变?
        let items: Vec<Vec<i32>> = vec![
            vec![0, 1],
            vec![1, 0],
            vec![-1, 0],
            vec![0, -1]
        ];
        let mut grid_mut: Vec<Vec<i32>> = vec![];
        for line in grid {
            grid_mut.push(vec![]);
            let currLen = grid_mut.len();
            grid_mut[currLen - 1].extend(line);
        }
        let n = grid_mut.len();
        let m = grid_mut[0].len();
        let mut ans = 0;
        for i in 0..n {
            let mut val = grid_mut[i][0];
            if val != 0 {
                ans = ans.max(Solution::solve(&mut grid_mut, 0, &items, i as i32, 0));
            }
            val = grid_mut[i][m - 1];
            if val != 0 {
                ans = ans.max(Solution::solve(&mut grid_mut, 0, &items, i as i32, (m - 1) as i32));
            }
        }
        for i in 1..m - 1 {
            let mut val = grid_mut[0][i];
            if val != 0 {
                ans = ans.max(Solution::solve(&mut grid_mut, 0, &items, 0, i as i32));
            }
            val = grid_mut[n - 1][i];
            if val != 0 {
                ans = ans.max(Solution::solve(&mut grid_mut, 0, &items, (n - 1) as i32, i as i32));
            }
        }
        return ans;
    }
    pub fn solve(grid: &mut Vec<Vec<i32>>, curr: i32, items: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        let val = grid[i as usize][j as usize];
        grid[i as usize][j as usize] = 0;
        let n_i32 = grid.len() as i32;
        let m_i32 = grid[0].len() as i32;
        let mut max = 0;
        for item in items {
            let nextI = i + item[0];
            let nextJ = j + item[1];
            if nextI < 0 || nextI >= n_i32 || nextJ < 0 || nextJ >= m_i32 {
                continue;
            }
            let nextI_usize = nextI as usize;
            let nextJ_usize = nextJ as usize;
            let nextVal = grid[nextI_usize][nextJ_usize];
            if nextVal != 0 {
                max = max.max(Solution::solve(grid, curr, items, nextI, nextJ));
            }
        }
        grid[i as usize][j as usize] = val;
        return val + curr + max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1219() {
        assert_eq!(24, Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]));
    }
}