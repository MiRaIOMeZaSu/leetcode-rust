use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        // 使用哈希表进行最后一步的判断
        let mut counts = HashMap::new();
        let mut ans: i32 = 0;
        let size: usize = nums.len();
        let mut i: usize = 3;
        while i < size {
            let count = counts.entry(nums[i]).or_insert(0);
            *count += 1;
            i += 1;
        }
        // 开始遍历
        let mut j: usize;
        let mut k: usize = 2;
        // i不能大于right - 1
        while k < size  - 1{
            j = k - 1;
            while j > 0 {
                i = j - 1;
                loop {
                    // 开始加和
                    let key = nums[i] + nums[j] + nums[k];
                    let temp_count = *counts.entry(key).or_insert(0);
                    if temp_count > 0{
                        ans += temp_count;
                    }
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }
                j -= 1;
            }
            let count = counts.entry(nums[k + 1]).or_insert(0);
            *count -= 1;
            k += 1;
        }
        return ans;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1995() {
        // assert_eq!(4, Solution::count_quadruplets(vec![1, 1, 1, 3, 5]));
        // assert_eq!(1, Solution::count_quadruplets(vec![1, 2, 3, 6]));
        // assert_eq!(1, Solution::count_quadruplets(vec![28,8,49,85,37,90,20,8]));
        assert_eq!(12, Solution::count_quadruplets(vec![100,63,81,1,36,17,82,52,62,1,81,6,38,94,56,76,54,49,54,71,27]));
    }
}