use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let items: Vec<Vec<i32>> = vec![
            vec![0, 1],
            vec![1, 0],
            vec![-1, 0],
            vec![0, -1]
        ];
        // 使用队列
        let m = is_water.len();
        let n = is_water[0].len();
        let mut ans: Vec<Vec<i32>> = vec![vec![-1; n as usize]; m as usize];
        let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    // 是水
                    ans[i][j] = 0;
                    queue.push_back(vec![i, j]);
                }
            }
        }
        while !queue.is_empty() {
            let point = queue.pop_front().unwrap();
            let i = point[0] as usize;
            let j = point[1];
            let curr = ans[i][j];
            for k in 0..4 {
                let next_i = i as i32 + items[k][0];
                let next_j = j as i32 + items[k][1];
                if next_i >= 0 && next_j >= 0 && next_i < m as i32 && next_j < n as i32 {
                    if ans[next_i as usize][next_j as usize] == -1 {
                        ans[next_i as usize][next_j as usize] = curr + 1;
                        queue.push_back(vec![next_i as usize, next_j as usize]);
                    }
                }
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1765() {
        assert_eq!(vec![vec![1, 0], vec![2, 1]], Solution::highest_peak(vec![vec![0, 1], vec![0, 0]]));
    }
}