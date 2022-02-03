pub struct Solution {}

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        // 重点,只由 a 和 b 组成
        // 重点,是子序列
        // 最多两次,最少一次
        // 判断是否为回文即可
        let size = s.len();
        let mut left: usize = 0;
        let bytes = s.as_bytes();
        let mut right: usize = size - 1;
        while left < right {
            let left_val = bytes[left];
            let right_val = bytes[right];
            if left_val != right_val {
                return 2;
            }
            left += 1;
            right -= 1;
        }
        return 1;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1332() {
        assert_eq!(2, Solution::remove_palindrome_sub(String::from("baabb")));
    }
}