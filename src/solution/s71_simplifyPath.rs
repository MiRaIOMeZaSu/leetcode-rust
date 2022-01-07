pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        // 逐个?如何回退?
        let size = path.len();
        let bytes = path.as_bytes();
        let mut arr_vec_l: Vec<u8> = vec![];
        let slash = '/' as u8;
        let point = '.' as u8;
        let mut i: usize = 0;
        while i < size {
            if bytes[i] == slash {
                if arr_vec_l.len() < 1 || arr_vec_l[arr_vec_l.len() - 1] != slash {
                    arr_vec_l.push(slash);
                }
                i += 1;
                continue;
            } else if bytes[i] == point {
                // 判断下一位
                if bytes[i - 1] == slash {
                    // 情况: /./
                    if i + 1 == size || bytes[i + 1] == slash {
                        i = i + 2;
                        continue;
                    } else if i + 1 < size && bytes[i + 1] == point && (i + 2 == size || bytes[i + 2] == slash) {
                        // 情况: /../ , 开始回退
                        if arr_vec_l.len() != 1 {
                            arr_vec_l.pop();
                            while arr_vec_l[arr_vec_l.len() - 1] != slash {
                                arr_vec_l.pop();
                            }
                        }
                        i += 3;
                        continue;
                    }
                }
            }
            arr_vec_l.push(bytes[i]);
            i += 1;
        }
        if arr_vec_l.len() != 1 && arr_vec_l[arr_vec_l.len() - 1] == slash {
            arr_vec_l.pop();
        }
        return String::from_utf8(arr_vec_l).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_71() {
        assert_eq!("/home",Solution::simplify_path("/home/".to_string()));
        assert_eq!("/", Solution::simplify_path("/../".to_string()));
        assert_eq!("/home/foo", Solution::simplify_path("/home//foo/".to_string()));
    }
}