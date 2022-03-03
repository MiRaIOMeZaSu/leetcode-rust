struct Solution {}

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        // 如果是
        if n == String::from("1") {
            return String::from("0");
        }
        let zero = '0' as u8;
        let nine = '9' as u8;
        let one = '1' as u8;
        let size = n.len();
        let mut ans: Vec<u8> = vec![zero; size];
        let bytes = n.as_bytes();
        // 从零到
        let center_index = (size + 1) / 2 - 1;
        let center_val = bytes[center_index];
        for i in 0..center_index {
            ans[i] = bytes[i];
            ans[size - i - 1] = bytes[i];
        }
        ans[center_index] = bytes[center_index];
        if size % 2 == 0 {
            ans[center_index + 1] = ans[center_index];
        }
        if bytes[0] == one && Solution::all_zero(bytes) && (bytes[size - 1] == one || bytes[size - 1] == zero) {
            // 如何处理
            // 减去2!
            // 实际上是减少一位的全9
            ans = vec![nine; size - 1];
        } else if Solution::is_palindromic(bytes) {
            // 假如本身就是回文该如何寻找相近的?
            // 使得最靠近中间的值发生变化!
            // 返回较小的那个
            // 假如中间的值为单个或双个0,或单个或多个9
            let mut center_really_val = n.as_str()[center_index..center_index + 1].parse::<i32>().unwrap();
            if center_val == zero {
                // 如何处理?
                // 假如遍历将存在9位的范围!
                // 简单加一
                center_really_val += 1;
            } else {
                center_really_val -= 1;
            }
            ans[center_index] = center_really_val.to_string().to_string().as_bytes()[0];
            if size % 2 == 0{
                ans[center_index + 1] = center_really_val.to_string().to_string().as_bytes()[0];
            }
        }
        return String::from_utf8(ans).unwrap();
    }
    pub fn all_zero(bytes: &[u8]) -> bool {
        let zero = '0' as u8;
        let size = bytes.len();
        let mut flag = true;
        for i in 1..size - 1 {
            if bytes[i] != zero {
                flag = false;
            }
        }
        return flag;
    }
    pub fn is_palindromic(bytes: &[u8]) -> bool {
        let mut right = bytes.len() - 1;
        let mut left: usize = 0;
        while left < right {
            if bytes[left] != bytes[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        return true;
    }
}


// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_564() {
        assert_eq!(String::from("11"), Solution::nearest_palindromic(String::from("12")));
        assert_eq!(String::from("9"), Solution::nearest_palindromic(String::from("10")));
        assert_eq!(String::from("99"), Solution::nearest_palindromic(String::from("100")));
        assert_eq!(String::from("99"), Solution::nearest_palindromic(String::from("101")));
        assert_eq!(String::from("121"), Solution::nearest_palindromic(String::from("123")));
        assert_eq!(String::from("121121"), Solution::nearest_palindromic(String::from("121335")));
    }
}