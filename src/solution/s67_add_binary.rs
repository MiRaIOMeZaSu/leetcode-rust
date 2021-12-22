pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        // 先转成数字再转成2进制
        let size_a = a.len();
        let size_b = b.len();
        let alt_b = size_a > size_b;
        let mut long_index;
        let mut short_index;
        let long_bytes;
        let short_bytes;
        if alt_b {
            long_index = size_a - 1;
            long_bytes = a.as_bytes();
            short_index = size_b - 1;
            short_bytes = b.as_bytes();
        } else {
            long_index = size_b - 1;
            long_bytes = b.as_bytes();
            short_index = size_a - 1;
            short_bytes = a.as_bytes();
        }
        let mut arr: Vec<u8> = Vec::new();
        // 开始遍历
        let mut next: u8 = 0;
        let pivot: u8 = 1.to_string().as_bytes()[0];
        loop {
            let mut curr: u8 = next;
            if long_bytes[long_index] == pivot {
                curr = curr + 1;
            }
            if short_bytes[short_index] == pivot {
                curr = curr + 1;
            }
            next = curr / 2;
            arr.push(curr % 2);
            if long_index != 0 {
                long_index = long_index - 1;
            }
            if short_index != 0 {
                short_index = short_index - 1;
            } else {
                break;
            }
        }
        if size_a != size_b {
            loop {
                let mut curr: u8 = next;
                if long_bytes[long_index] == pivot {
                    curr = curr + 1;
                }
                next = curr / 2;
                arr.push(curr % 2);
                if long_index != 0 {
                    long_index = long_index - 1;
                } else {
                    break;
                }
            }
        }
        if next == 1 {
            arr.push(1);
        }
        return arr.iter().rev().map(|x| x.to_string()).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("100", Solution::add_binary(String::from("11"),String::from("1")));
    }
}