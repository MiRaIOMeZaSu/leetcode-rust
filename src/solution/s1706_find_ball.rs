use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        // 如何更改状态
        // 1 - \
        // -1 - /
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        let height = grid.len();
        let count = grid[0].len();
        let mut ans: Vec<i32> = vec![-1; grid[0].len()];
        for i in 0..count {
            map.entry(i).or_insert(vec![]).push(i);
        }
        for i in 0..height {
            let mut next_map: HashMap<usize, Vec<usize>> = HashMap::new();
            for (k, v) in map {
                // k处该如何处理
                let block = grid[i][k];
                let mut left = 1;
                let mut right = -1;
                if k >= 1 {
                    left = grid[i][k - 1];
                }
                if k  < count - 1 {
                    right = grid[i][k + 1];
                }
                if block == 1 {
                    // 往右走
                    if right == 1 {
                        for num in v {
                            next_map.entry(k + 1).or_insert(vec![]).push(num);
                        }
                    }
                } else {
                    // 往左走
                    if left == -1 {
                        for num in v {
                            next_map.entry(k - 1).or_insert(vec![]).push(num);
                        }
                    }
                }
            }
            map = next_map;
        }
        for (k, v) in map {
            for i in v {
                ans[i] = k as i32;
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
    fn test_1706() {
        // Solution::gray_code(2);
        assert_eq!(vec![0, 1, 2, 3, 4, -1], Solution::find_ball(vec![vec![1, 1, 1, 1, 1, 1], vec![-1, -1, -1, -1, -1, -1], vec![1, 1, 1, 1, 1, 1], vec![-1, -1, -1, -1, -1, -1]]));
    }
}