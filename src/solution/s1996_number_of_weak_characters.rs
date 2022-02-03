use std::collections::HashMap;

pub struct Solution {}

pub fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    return b;
}

impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        // 先排序
        let mut properties_mut = properties;
        let size = properties_mut.len();
        let mut ans = 0;
        // 通过attack排序
        properties_mut.sort_by_key(|k| k[0]);
        let mut attack_to_min_defence: HashMap<i32, i32> = HashMap::new();
        let mut i: usize = size;
        let mut last_max = i32::MIN;
        let mut next_greater = vec![properties_mut[size - 1][0]];
        let mut index = 0;
        while i >= 1 {
            let val = *attack_to_min_defence.entry(properties_mut[i - 1][0]).or_insert(properties_mut[i - 1][1]);
            last_max = max(val, max(properties_mut[i - 1][1], last_max));
            attack_to_min_defence.insert(properties_mut[i - 1][0], last_max);
            if properties_mut[i - 1][0] != next_greater[index] {
                next_greater.push(properties_mut[i - 1][0]);
                index += 1;
            }
            i-=1;
        }
        i = size;
        index = 0;
        while i >= 1 {
            if properties_mut[i - 1][0] == next_greater[0] {
                i-=1;
                continue;
            }
            if properties_mut[i - 1][0] != next_greater[index] {
                index += 1;
            }
            let val = *attack_to_min_defence.entry(next_greater[index - 1]).or_insert(i32::MAX);
            if val > properties_mut[i - 1][1] {
                ans += 1;
            }
            i -= 1;
        }
        return ans;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1996() {
        // assert_eq!(1, Solution::number_of_weak_characters(vec![vec![3, 3], vec![2, 2]]));
        assert_eq!(0, Solution::number_of_weak_characters(vec![vec![5,5],vec![6,3],vec![3,6]]));
    }
}