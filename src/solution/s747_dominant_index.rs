pub struct Solution {}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    b
}

fn min(a: i32, b: i32) -> i32 {
    if a > b {
        return b;
    }
    a
}

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        // 获取前两大的数字
        let size = nums.len();
        if size == 1 {
            return 0;
        }
        let mut first: i32;
        let mut firstIndex: i32;
        let mut second: i32;
        if nums[0] > nums[1] {
            first = nums[0];
            second = nums[1];
            firstIndex = 0;
        } else {
            first = nums[1];
            second = nums[0];
            firstIndex = 1;
        }
        for i in 2..size {
            if nums[i] > second {
                if nums[i] > first {
                    firstIndex = i as i32;
                    second = first;
                    first = nums[i];
                    continue;
                }
                second = nums[i];
            }
        }
        if first >= second * 2 {
            return firstIndex;
        }
        return -1;
    }
}