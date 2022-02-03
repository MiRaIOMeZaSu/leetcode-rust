use std::collections::HashMap;

pub struct Solution {}

pub fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    }
    return b;
}

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        // 使用动态规划
        // 先寻找所有能跳的地方
        let mut map: HashMap<i32, i32> = HashMap::new();
        let size = arr.len();
        let mut dp: Vec<i32> = vec![i32::MAX - 1; size];
        map.insert(arr[0], 0);
        Solution::solve(&arr, &mut dp, &mut map, 0, 0);
        for i in 1..size {
            let map_val = *map.entry(arr[i]).or_insert(i32::MAX - 1);
            let mut near = i32::MAX;
            if dp[i] > map_val + 1 {
                near = map_val + 1;
            }
            if i + 1 < size {
                near = min(dp[i - 1], dp[i + 1]) + 1;
            } else {
                near = dp[i - 1] + 1;
            }
            if dp[i] > near {
                dp[i] = near;
            }
            let next_val = dp[i];
            Solution::set_map_min(&arr, i, &mut map, &dp);
            Solution::solve(&arr, &mut dp, &mut map, i, next_val);
        }
        return dp[size - 1];
    }
    fn set_map_min(arr: &Vec<i32>, index: usize, map: &mut HashMap<i32, i32>, dp: &Vec<i32>) {
        let min_val = *map.entry(arr[index]).or_insert(i32::MAX - 1);
        if dp[index] < min_val {
            map.insert(arr[index], dp[index]);
        }
    }
    fn solve(arr: &Vec<i32>, dp: &mut Vec<i32>, map: &mut HashMap<i32, i32>, index: usize, val: i32) {
        let size = dp.len();
        dp[index] = val;
        Solution::set_map_min(arr, index, map, dp);
        let curr = dp[index];
        // 使用左右填色法
        let mut map_val: i32;
        if index != 0 {
            // 往左边走
            map_val = *map.entry(arr[index - 1]).or_insert(i32::MAX - 1);
            if dp[index - 1] > map_val + 1 {
                dp[index - 1] = map_val + 1;
            }
            if dp[index - 1] > curr + 1 {
                Solution::solve(arr, dp, map, index - 1, curr + 1);
            }
        }
        if index != size - 1 {
            // 往右边走
            map_val = *map.entry(arr[index + 1]).or_insert(i32::MAX - 1);
            if dp[index + 1] > map_val + 1 {
                dp[index + 1] = map_val + 1;
            }
            if dp[index + 1] > curr + 1 {
                Solution::solve(arr, dp, map, index + 1, curr + 1);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1345() {
        assert_eq!(3, Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]));
    }
}


