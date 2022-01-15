pub struct Solution {}

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut times = n / 7;
        let rest = n % 7;
        let mut week = 0;
        let mut ans = 0;
        for i in 1..8 {
            week += i;
        }
        for i in 0..times {
            ans += week + i * 7;
        }
        for i in 1..rest + 1 {
            ans += times + i;
        }
        return ans;
    }
}