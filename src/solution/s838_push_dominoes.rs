struct Solution {}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        // 在两个间隔之间选择
        let mut indexs: Vec<usize> = vec![];

        let size = dominoes.len();
        let mut bytes = dominoes.as_bytes().to_vec();
        let dot: u8 = '.' as u8;
        let left: u8 = 'L' as u8;
        let right: u8 = 'R' as u8;
        for i in 0..size {
            let byte = bytes[i];
            if dot != byte {
                indexs.push(i);
            }
        }
        if indexs.len() == 0 {
            return dominoes;
        }
        let move_count = indexs.len();
        if bytes[indexs[0]] == left {
            for i in 0..indexs[0] {
                bytes[i] = left;
            }
        }
        for i in 0..move_count - 1 {
            // i 到 i + 1 间的被推牌
            // 对方向的判定
            let mut l = indexs[i] + 1;
            let mut r = indexs[i + 1] - 1;
            let left_byte = bytes[l - 1];
            let right_byte = bytes[r + 1];
            if left_byte == left && right_byte == left {
                for j in l..r + 1 {
                    bytes[j] = left;
                }
            }
            if left_byte == right && right_byte == left {
                while l < r {
                    bytes[l] = right;
                    bytes[r] = left;
                    l += 1;
                    r -= 1;
                }
            }
            if left_byte == right && right_byte == right {
                for j in l..r + 1 {
                    bytes[j] = right;
                }
            }
        }
        if bytes[indexs[move_count - 1]] == right {
            for i in indexs[move_count - 1] + 1..size {
                bytes[i] = right;
            }
        }
        return String::from_utf8(bytes).unwrap();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_838() {
        // assert_eq!(String::from("."), Solution::push_dominoes(String::from(".")));
        assert_eq!(String::from("RRR.L"), Solution::push_dominoes(String::from("R.R.L")));
    }
}