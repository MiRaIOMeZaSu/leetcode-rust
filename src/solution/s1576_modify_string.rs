pub struct Solution {}

impl Solution {
    pub fn modify_string(s: String) -> String {
        // 问号question_mark
        let question_mark_u8 = '?' as u8;
        let size = s.len();
        let chars = ['a' as u8, 'b' as u8, 'c' as u8];
        let s_bytes = s.as_bytes();
        let mut ans = "".to_string();
        let mut front: u8 = question_mark_u8;
        let mut back: u8 = question_mark_u8;
        for i in 0..size {
            if s_bytes[i] == question_mark_u8 {
                if i + 1 < size {
                    back = s_bytes[i + 1];
                }
                for ch in chars {
                    if ch != front && ch != back {
                        ans.push(char::from(ch));
                        front = ch;
                        break;
                    }
                }
            }else{
                ans.push(char::from(s_bytes[i]));
                front = s_bytes[i];
            }

        }
        return ans.to_string();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1576() {
        Solution::modify_string("??yw?ipkj?".to_string());
        Solution::modify_string("j?qg??b".to_string());
    }
}