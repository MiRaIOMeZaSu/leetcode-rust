pub struct Solution {}

impl Solution {
    const items: [[i32; 2]; 4] = [
        [-1, 0],
        [0, -1],
        [1, 0],
        [0, 1],
    ];
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        // 深度优先法
        // 剪枝: 按边判断,判断剩余步数能否走出去
        return Solution::solve(start_row, start_column, max_move, m, n);
    }

    fn solve(i: i32, j: i32, rest_move: i32, m: i32, n: i32) -> i32 {
        // 剩余步骤是否足够
        if i < 0 || j < 0 || i >= m || j >= n {
            return 1;
        }
        if (i + 1 > rest_move && m - i > rest_move) && (j > rest_move && n - j > rest_move) {
            // 步数不够
            return 0;
        }
        let mut ans = 0;
        if rest_move > 0 {
            for k in 0..4 {
                ans += Solution::solve(Solution::items[k][0] + i, Solution::items[k][1] + j, rest_move - 1, m, n);
            }
        }
        return ans;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_576() {
        assert_eq!(6, Solution::find_paths(2, 2, 2, 0, 0));
    }
}