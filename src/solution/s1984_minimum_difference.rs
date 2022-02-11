struct Solution {}

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        // 使用滑动窗口
        let mut nums_mut: Vec<i32> = nums;
        nums_mut.sort();
        let mut ans: i32 = i32::MAX;
        if k == 0 {
            return 0;
        }
        // 长度为k的窗口
        let size = nums_mut.len();
        let mut start: usize = 0;
        let mut end: usize = start + k as usize - 1;
        while end < size {
            ans = ans.min(nums_mut[end] - nums_mut[start]);
            start += 1;
            end += 1;
        }
        return ans;
    }
}