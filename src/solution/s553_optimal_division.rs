use std::ops::Add;

struct Solution {}

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        // 保证第一个数只能除以最小的那个数?
        let mut ans: String = String::new();
        let size = nums.len();
        let slash = "/";
        let left = "(";
        let right = ")";
        ans = ans.add(nums[0].to_string().as_str());
        if size == 1 {
            return ans;
        }
        ans = ans.add(slash);
        if size == 2 {
            ans = ans.add(nums[1].to_string().as_str());
            return ans;
        }
        ans = ans.add(left);
        for i in 1..size - 1 {
            ans = ans.add(nums[i].to_string().as_str());
            ans = ans.add(slash);
        }
        ans = ans.add(nums[size - 1].to_string().as_str());
        ans = ans.add(right);
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_553() {
        assert_eq!(String::from("1000/(100/10/2)"), Solution::optimal_division(vec![1000, 100, 10, 2]));
    }
}