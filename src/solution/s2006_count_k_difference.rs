use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        // 使用哈希表存储数量
        let mut ans = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let size = nums.len();
        for i in 0..size {
            let num = nums[i];
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }
        // 开始计算
        for i in 0..size {
            let num = nums[i];
            let count = map.entry(num).or_insert(0);
            *count -= 1;
            let l_target = num - k;
            let r_target = num + k;
            ans += *map.entry(l_target).or_insert(0);
            ans += *map.entry(r_target).or_insert(0);
        }
        return ans;
    }
}