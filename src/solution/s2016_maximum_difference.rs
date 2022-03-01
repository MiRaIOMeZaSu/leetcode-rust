struct Solution {}

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        let mut right_max: Vec<i32> = vec![0; size];
        let mut ans = -1;
        let mut index = size - 1;
        let mut currMax = nums[index];
        while index >= 1 {
            currMax = currMax.max(nums[index]);
            right_max[index] = currMax;
            index -= 1;
        }
        let mut left_min = nums[0];
        for i in 0..size - 1 {
            left_min = left_min.min(nums[i]);
            if right_max[i + 1] > left_min {
                ans = ans.max(right_max[i + 1] - left_min);
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2016() {
        assert_eq!(57, Solution::maximum_difference(vec![2, 59, 37, 57, 10, 30]));
    }
}