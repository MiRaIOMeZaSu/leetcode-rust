pub struct Solution {}

impl Solution {
    pub const PIVOT: i32 = 1000000007;
    pub fn count_vowel_permutation(n: i32) -> i32 {
        // 简单的动态规划
        // 规则:
        // e,i,u -> a;
        // a,i -> e;
        // e,o -> i;
        // i -> o;
        // o,i -> u;
        let mut curr: Vec<i32> = vec![1; 5];
        let mut next: Vec<i32> = vec![1; 5];
        for _ in 1..n as usize{
            next[0] = curr[1] + curr[2];
            next[0] %= Solution::PIVOT;
            next[0] += curr[4];
            next[1] = curr[0] + curr[2];
            next[2] = curr[1] + curr[3];
            next[3] = curr[2];
            next[4] = curr[2] + curr[3];
            for j in 0..5 {
                curr[j] = next[j] % Solution::PIVOT;
            }
        }
        let mut ans: i32 = 0;
        for i in 0..5 {
            ans += curr[i];
            ans %= Solution::PIVOT;
        }
        return ans;
    }
}