struct Solution {}

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        // 翻转字符串
        let pivot = '-' as u8;
        let chars = s.as_bytes();
        let size = s.len();
        let mut chars_vec: Vec<u8> = chars.to_vec();
        let mut index = size - 1;
        let mut index_vec = 0;
        loop {
            if Solution::isAlphabet(chars[index]) {
                while !Solution::isAlphabet(chars_vec[index_vec]) {
                    index_vec += 1;
                }
                chars_vec[index_vec] = chars[index];
                index_vec += 1;
            }
            if index == 0 {
                break;
            }
            index -= 1;
        }
        return String::from_utf8(chars_vec).unwrap();
    }
    pub fn isAlphabet(char: u8) -> bool {
        if char >= 'a' as u8 && char <= 'z' as u8 {
            return true;
        }
        if char >= 'A' as u8 && char <= 'Z' as u8 {
            return true;
        }
        return false;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_917() {
        // Solution::gray_code(2);
        assert_eq!(String::from("dc-ba"), Solution::reverse_only_letters(String::from("ab-cd")));
    }
}