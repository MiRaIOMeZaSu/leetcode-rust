struct Solution {}

impl Solution {
    const ITEMS: [[i32; 2]; 8] = [
        [-2, 1],
        [-2, -1],
        [-1, -2],
        [-1, 2],
        [2, 1],
        [2, -1],
        [1, -2],
        [1, 2]
    ];
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        // 遍历?
        // 统计每个位置在一定剩余步数的情况下有多大的可能
        let mut graph: Vec<Vec<Vec<f64>>> = vec![vec![vec![-1.0; (k + 1) as usize]; n as usize]; n as usize];
        if k == 0 {
            if Solution::satisfy(n, row, column) {
                return 1.0;
            } else {
                return 0.0;
            }
        }
        return Solution::solve(n, k, row, column, &mut graph);
    }
    pub fn solve(n: i32, k: i32, row: i32, column: i32, graph: &mut Vec<Vec<Vec<f64>>>) -> f64 {
        // 能留
        if k == 0 {
            return 1.0;
        }
        let row_usize = row as usize;
        let column_usize = column as usize;
        let k_usize = k as usize;
        if graph[row_usize][column_usize][k_usize] >= 0.0 {
            return graph[row_usize][column_usize][k_usize];
        }
        let mut ans: f64 = 0.0;
        for item in Solution::ITEMS {
            let next_i: i32 = row + item[0];
            let next_j: i32 = column + item[1];
            if Solution::satisfy(n, next_i, next_j) {
                ans += Solution::solve(n, k - 1, next_i, next_j, graph) / 8.0;
            }
        }
        graph[row_usize][column_usize][k_usize] = ans;
        return ans;
    }

    pub fn satisfy(n: i32, row: i32, column: i32) -> bool {
        if row < 0 || column < 0 || row >= n || column >= n {
            return false;
        }
        return true;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_688() {
        assert_eq!(1.0, Solution::knight_probability(1, 0, 0, 0));
        assert_eq!(0.0625, Solution::knight_probability(3, 2, 0, 0));
    }
}