pub struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        // 判断左右括号
        let bytes = s.as_bytes();
        let mut ans = 0;
        let mut curr = 0;
        let left = '(' as u8;
        let right = ')' as u8;
        for i in 0..bytes.len() {
            if bytes[i] == left {
                curr += 1;
            } else if bytes[i] == right {
                curr -= 1;
            }
            if curr > ans {
                ans = curr;
            }
        }
        return ans;
    }
}