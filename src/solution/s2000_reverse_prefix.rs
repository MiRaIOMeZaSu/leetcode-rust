struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut size = word.len();
        let povit = ch as u8;
        let bytes = word.as_bytes();
        let mut border: usize = 0;
        for i in 0..size {
            if bytes[i] == povit {
                border = i;
                break;
            }
        }
        let mut prex = &bytes[0..border + 1];
        let mut suffix = &bytes[border + 1..size];
        let mut ans: String = String::from_utf8_lossy(&prex).to_string();
        ans = ans.chars().rev().collect::<String>();
        ans.push_str(String::from_utf8_lossy(&suffix).as_ref());
        return ans;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1763() {
        assert_eq!(String::from("dcbaefd"), Solution::reverse_prefix(String::from("abcdefd"), 'd'));
    }
}