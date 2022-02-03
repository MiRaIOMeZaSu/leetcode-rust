use std::collections::{HashSet, HashMap};

pub struct Solution {}

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        // 两个集合的差集
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        for word in s1.split_whitespace() {
            let count = map1.entry(word).or_insert(0);
            *count += 1;
        }
        for word in s2.split_whitespace() {
            let count = map2.entry(word).or_insert(0);
            *count += 1;
        }
        let set1 = Solution::filter(&map1);
        let set2 = Solution::filter(&map2);
        let set1_ge_one = Solution::filter_gt_one(&map1);
        let set2_ge_one = Solution::filter_gt_one(&map2);
        let diff1 = set1_ge_one.difference(&set2).map(String::from).collect::<HashSet<_>>();
        let diff2 = set2_ge_one.difference(&set1).map(String::from).collect::<HashSet<_>>();
        let ans = diff1.union(&diff2).map(String::from).collect::<Vec<_>>();
        return ans;
    }
    fn filter_gt_one(map: &HashMap<&str, i32>) -> HashSet<String> {
        return map.iter().filter(|&(k, v)| *v == 1).map(|(k, v)| *k).map(String::from).collect::<HashSet<_>>();
    }
    fn filter(map: &HashMap<&str, i32>) -> HashSet<String> {
        return map.iter().map(|(k, v)| *k).map(String::from).collect::<HashSet<_>>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_884() {
        assert_eq!(vec![String::from("sweet"), String::from("sour")], Solution::uncommon_from_sentences(
            String::from("this apple is sweet"),
            String::from("this apple is sour"),
        ));
    }
}