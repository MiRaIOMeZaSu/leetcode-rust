struct Solution {}

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        // 简单的遍历,存储最小最大值
        let size = nums.len();
        let mut ans: i64 = 0;
        for i in 0..size {
            let mut min = i32::MAX;
            let mut max = i32::MIN;
            for j in i..size {
                // 从i到j范围中间的
                min = min.min(nums[j]);
                max = max.max(nums[j]);
                ans += (max - min) as i64;
            }
        }
        return ans;
    }
}