use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        // 使用map
        let mut indexs: HashMap<i32, i32> = HashMap::new();
        let size = nums.len();
        for i in 0..size {
            let i_i32 = i as i32;
            let index = *indexs.entry(nums[i]).or_insert(-1);
            indexs.insert(nums[i], i_i32);
            if index == -1 {
                continue;
            }
            if i_i32 - index <= k {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_219() {
        assert_eq!(true, Solution::contains_nearby_duplicate(vec![1,2,3,1],3));
    }
}