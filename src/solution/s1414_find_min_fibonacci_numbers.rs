struct Solution {}

impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        // 动态规划
        // 如何保证每次能加的是最小的?
        let mut arr: Vec<i32> = vec![1, 1];
        let mut index = 1;
        let mut ans = 0;
        while arr[index] <= k {
            arr.push(arr[index - 1] + arr[index]);
            index += 1;
        }
        let mut currK = k;
        while index > 0 {
            if arr[index] <= currK {
                currK -= arr[index];
                ans += 1;
            }
            index -= 1;
        }
        return ans;
    }
}