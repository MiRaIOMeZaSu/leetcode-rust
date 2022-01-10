use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        // 使用哈希表从头遍历
        let mut counts: HashMap<i32, i32> = HashMap::new();
        let mut i: usize = 0;
        let size: usize = hand.len();
        if size as i32 % group_size != 0 {
            return false;
        }
        let mut distinct_arr: Vec<i32> = vec![];
        while i < size {
            let count = counts.entry(hand[i]).or_insert(0);
            if *count == 0 {
                distinct_arr.push(hand[i]);
            }
            *count += 1;
            i += 1;
        }
        distinct_arr.sort();
        let mut index: usize = 0;
        // 开始做减法
        let group_size_usize = usize::from(group_size as u16);
        let mut time = size as i32 / group_size;
        while time > 0 {
            while *counts.entry(distinct_arr[index]).or_insert(0) == 0 {
                index += 1;
            }
            let mut curr_index: usize = index;
            i = 0;
            while i < group_size_usize {
                if curr_index >= distinct_arr.len() {
                    return false;
                }
                let count = counts.entry(distinct_arr[curr_index]).or_insert(0);
                if *count > 0 && (curr_index == index || distinct_arr[curr_index] == distinct_arr[curr_index - 1] + 1) {
                    *count -= 1;
                } else {
                    return false;
                }
                curr_index += 1;
                i += 1;
            }
            time -= 1;
        }
        return true;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_846() {
        // assert_eq!(true, Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3));
        // assert_eq!(false, Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
        // assert_eq!(false, Solution::is_n_straight_hand(vec![8, 10, 12], 3));
        assert_eq!(true, Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3], 2));
    }
}