pub struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut ans = 0;
        let mut curr = num;
        while curr != 0 {
            if curr % 2 == 1 {
                curr -= 1;
            } else {
                curr /= 2;
            }
            ans += 1;
        }
        return ans;
    }
}