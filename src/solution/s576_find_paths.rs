pub struct Solution {}

impl Solution {
    const PIVOT: i32 = 1000000007;
    const ITEMS: [[i32; 2]; 4] = [
        [-1, 0],
        [0, -1],
        [1, 0],
        [0, 1],
    ];
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        // 深度优先法
        // 剪枝: 按边判断,判断剩余步数能否走出去
        // 变化的值只有三个
        let mut memory: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; (max_move + 1) as usize]; (n + 1) as usize]; (m + 1) as usize];
        return Solution::solve(start_row, start_column, max_move, m, n, &mut memory);
    }

    fn solve(i: i32, j: i32, rest_move: i32, m: i32, n: i32, memory: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        // 剩余步骤是否足够
        if i < 0 || j < 0 || i >= m || j >= n {
            return 1;
        }
        if (i + 1 > rest_move && m - i > rest_move) && (j > rest_move && n - j > rest_move) {
            // 步数不够
            return 0;
        }
        let i_usize = i as usize;
        let j_usize = j as usize;
        let rest_move_usize = rest_move as usize;
        if memory[i_usize][j_usize][rest_move_usize] != -1 {
            return memory[i_usize][j_usize][rest_move_usize];
        }
        let mut ans = 0;
        if rest_move > 0 {
            for k in 0..4 {
                ans += Solution::solve(Solution::ITEMS[k][0] + i, Solution::ITEMS[k][1] + j, rest_move - 1, m, n, memory);
                if ans >= Solution::PIVOT {
                    ans %= Solution::PIVOT;
                }
            }
        }
        if ans >= Solution::PIVOT {
            ans %= Solution::PIVOT;
        }
        memory[i_usize][j_usize][rest_move_usize] = ans;
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
        assert_eq!(12, Solution::find_paths(1, 3, 3, 0, 1));
        assert_eq!(914783380, Solution::find_paths(8, 50, 23, 5, 26));
        assert_eq!(
            43458, Solution::find_paths(8, 4, 10, 5, 0));
    }
}