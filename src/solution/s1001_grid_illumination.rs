use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    const ITEMS: [[i64; 2]; 9] = [
        [-1, -1],
        [1, 1],
        [1, -1],
        [-1, 1],
        [-1, 0],
        [0, -1],
        [1, 0],
        [0, 1],
        [0, 0]
    ];
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 存储不同的情况
        // 如何在集合中删除指定结果
        // 使用i64存储
        let size = lamps.len();
        let mut lamps_hashmap: HashMap<i64, Vec<i32>> = HashMap::new();
        let mut col: HashMap<i32, HashSet<i64>> = HashMap::new();
        let mut row: HashMap<i32, HashSet<i64>> = HashMap::new();
        // key = item[0] - item[1]
        let mut leftToTop: HashMap<i32, HashSet<i64>> = HashMap::new();
        // key = item[1] - item[0]
        let mut topToRight: HashMap<i32, HashSet<i64>> = HashMap::new();
        let mut ans: Vec<i32> = vec![];
        for lamp in lamps {
            let i_i32 = lamp[0];
            let j_i32 = lamp[1];
            let i: i64 = lamp[0] as i64;
            let j: i64 = lamp[1] as i64;
            let key: i64 = (i << 32) + j;
            let keyleftToTop = i_i32 + j_i32;
            let keytopToRight = j_i32 - i_i32;
            col.entry(i_i32).or_insert(HashSet::new()).insert(key);
            row.entry(j_i32).or_insert(HashSet::new()).insert(key);
            leftToTop.entry(keyleftToTop).or_insert(HashSet::new()).insert(key);
            topToRight.entry(keytopToRight).or_insert(HashSet::new()).insert(key);
            lamps_hashmap.insert(key, lamp);
        }
        for query in queries {
            let i_i32 = query[0];
            let j_i32 = query[1];
            let i: i64 = i_i32 as i64;
            let j: i64 = j_i32 as i64;
            let key: i64 = (i << 32) + j;
            let keyleftToTop = i_i32 + j_i32;
            let keytopToRight = j_i32 - i_i32;
            let gloom = col.entry(i_i32).or_insert(HashSet::new()).len() > 0
                || row.entry(j_i32).or_insert(HashSet::new()).len() > 0
                || leftToTop.entry(keyleftToTop).or_insert(HashSet::new()).len() > 0
                || topToRight.entry(keytopToRight).or_insert(HashSet::new()).len() > 0;
            if gloom
            {
                ans.push(1);
            } else {
                ans.push(0);
            }
            // 开始关灯
            // 遍历8个方向
            for item in Solution::ITEMS {
                let i_i32_temp = query[0] + item[0] as i32;
                let j_i32_temp = query[1] + item[1] as i32;
                let i_temp: i64 = i + item[0];
                let j_temp: i64 = j + item[1];
                let key: i64 = (i_temp << 32) + j_temp;
                let keyleft_to_top = i_i32_temp + j_i32_temp;
                let keytop_to_right = j_i32_temp - i_i32_temp;
                if lamps_hashmap.contains_key(&key) {
                    lamps_hashmap.remove(&key);
                    col.entry(i_i32_temp).or_insert(HashSet::new()).remove(&key);
                    row.entry(j_i32_temp).or_insert(HashSet::new()).remove(&key);
                    leftToTop.entry(keyleft_to_top).or_insert(HashSet::new()).remove(&key);
                    topToRight.entry(keytop_to_right).or_insert(HashSet::new()).remove(&key);
                }
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
    fn test_1001() {
        // assert_eq!(vec![1, 1], Solution::grid_illumination(5, vec![vec![0, 0], vec![4, 4]], vec![vec![1, 1], vec![1, 1]]));
        // assert_eq!(vec![1, 0], Solution::grid_illumination(5, vec![vec![0, 0], vec![4, 4]], vec![vec![1, 1], vec![1, 0]]));
        // assert_eq!(vec![1, 1, 0], Solution::grid_illumination(5, vec![vec![0, 0], vec![0, 4]], vec![vec![0, 4], vec![0, 1], vec![1, 4]]));
        assert_eq!(vec![1, 0], Solution::grid_illumination(6, vec![vec![1, 1]], vec![vec![2, 0], vec![1, 0]]));
    }
}
