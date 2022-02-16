use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        // 统计出现的数量和出现的字符数量
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;
        let pairs_size = pairs.len() as i32;
        for pair in pairs {
            for num in pair {
                let count = map.entry(num).or_insert(0);
                *count += 1;
            }
        }
        let distinct_size = map.len() as i32;
        // 要求的数量并不是不同根元素的数量, 而是可能产生的不同的树的数量
        for (k, v) in map {
            if v + 1 == distinct_size {
                ans += 1;
                if ans > 1 {
                    return ans;
                }
                if ans > 0 && pairs_size > v {
                    return 2;
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
    fn test_1405() {
        assert_eq!(1, Solution::check_ways(vec![vec![3, 5], vec![4, 5], vec![2, 5], vec![1, 5], vec![1, 4], vec![2, 4]]));
        // assert_eq!(2, Solution::check_ways(vec![vec![1, 5], vec![1, 3], vec![2, 3], vec![2, 4], vec![3, 5], vec![3, 4]]));
    }
}