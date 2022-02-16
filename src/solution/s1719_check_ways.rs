use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        // 统计出现的数量和出现的字符数量(预判断)
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;
        let pair_size = pairs.len() as i32;
        for pair in pairs {
            for num in pair {
                let count = map.entry(num).or_insert(0);
                *count += 1;
            }
        }
        let distinct_size = map.len() as i32;
        let mut count_to_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (k, v) in map {
            let count = count_to_map.entry(v).or_insert(vec![]);
            count.push(k);
            if v + 1 == distinct_size {
                ans += 1;
            }
            if ans > 1 {
                return 2;
            }
        }
        // 求并集, 属于同一个并集的数字为同一条从上至下的路径上的
        // 如果只存在唯一的根则进一步判断
        // 如果不能完整串成一条路径, 则说明存在多种方案
        // 每次添加剩余值最多的值, 以此构造树, 什么情况下构造不下去?
        // 当两个相连节点为同一个父节点节点的子节点时
        if ans != 1 {
            return ans;
        }
        let mut rest = pair_size - distinct_size + 1 + 1;
        if count_to_map.entry(rest).or_insert(vec![]).len() == 0 {
            return 2;
        }
        return 1;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1405() {
        assert_eq!(1, Solution::check_ways(vec![vec![3, 5], vec![4, 5], vec![2, 5], vec![1, 5], vec![1, 4], vec![2, 4]]));
        assert_eq!(2, Solution::check_ways(vec![vec![1, 5], vec![1, 3], vec![2, 3], vec![2, 4], vec![3, 5], vec![3, 4]]));
        assert_eq!(0, Solution::check_ways(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 4], vec![2, 5], vec![3, 5]]))
        // 删除所有3, 剩余[1,5],[2,4]
    }
}