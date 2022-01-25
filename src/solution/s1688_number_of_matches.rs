pub struct Solution {}

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        // 不断除2
        let mut ans: i32 = 0;
        let mut n_mut: i32 = n;
        while n_mut != 1 {
            let times = n_mut >> 1;
            ans += times;
            if n_mut % 2 == 0 {
                n_mut = times;
            } else {
                n_mut = times + 1;
            }
        }
        return ans;
    }
}