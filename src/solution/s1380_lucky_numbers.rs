struct Solution {}

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        // 存储每一行的最小值
        // 存储每一列的最大值
        let mut row: Vec<i32> = vec![];
        let mut col: Vec<i32> = vec![];
        let mut ans: Vec<i32> = vec![];
        let m: usize = matrix.len();
        let n: usize = matrix[0].len();
        for i in 0..m {
            let mut min = i32::MAX;
            for j in 0..n {
                min = min.min(matrix[i][j]);
            }
            row.push(min);
        }
        for j in 0..n {
            let mut max = 0;
            for i in 0..m {
                max = max.max(matrix[i][j]);
            }
            col.push(max);
        }
        for i in 0..m {
            for j in 0..n {
                if row[i] == col[j] {
                    ans.push(row[i]);
                }
            }
        }
        return ans;
    }
}