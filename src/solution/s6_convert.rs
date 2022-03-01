struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // 使用下标计算?
        // 对于开头和结尾,特殊处理
        // 两个一组
        if num_rows == 1 {
            return s;
        }
        let chars = s.as_bytes();
        let size = s.len();
        let mut ans: Vec<u8> = vec![];
        let mut visited: Vec<bool> = vec![false; size];
        let mut gap: usize = (num_rows as usize) * 2 - 2;
        let gap_unmut: usize = gap;
        let mut count = 0;
        while count < size {
            let mut index = 0;
            for j in 0..size {
                if !visited[j] {
                    index = j;
                    break;
                }
            }
            // 开始计算
            while index < size {
                ans.push(chars[index]);
                count += 1;
                visited[index] = true;
                if index + gap < size && gap < gap_unmut && gap != 0 {
                    ans.push(chars[index + gap]);
                    visited[index + gap] = true;
                    count += 1;
                }
                index += gap_unmut;
            }
            if gap >= 2 {
                gap -= 2;
            }
        }
        return String::from_utf8(ans).unwrap();
    }
}


// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        // assert_eq!(String::from("PAHNAPLSIIGYIR"), Solution::convert(String::from("PAYPALISHIRING"), 3));
        assert_eq!(String::from("AB"), Solution::convert(String::from("AB"), 2));
    }
}