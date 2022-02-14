struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut ans: i32 = 0;
        for i in nums {
            ans ^= i;
        }
        return ans;
    }
}