pub struct Solution {}

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        // 重点是如何比较满足条件
        let bytes_a = a.as_bytes();
        let bytes_b = b.as_bytes();
        let mut char_count_a: [i32; 26] = [0; 26];
        let mut char_count_bsize_a: [i32; 26] = [0; 26];
        let mut char_count_b: [i32; 26] = [0; 26];
        let size_a = a.len();
        let size_b = b.len();
        let pivot: usize = usize::from(String::from("a").as_bytes()[0]);
        let mut i: usize = 0;
        while i < size_a {
            char_count_a[usize::from(bytes_a[i]) - pivot] += 1;
            i += 1;
        }
        i = 0;
        while i < size_a && i < size_b {
            char_count_bsize_a[usize::from(bytes_b[i]) - pivot] += 1;
            i += 1;
        }
        i = 0;
        // 以单个单词为单位判断
        while size_b > size_a && i < 26 {
            if char_count_a[i] != char_count_bsize_a[i] {
                return -1;
            }
            i += 1;
        }
        i = 0;
        while i < size_b {
            char_count_b[usize::from(bytes_b[i]) - pivot] += 1;
            i += 1;
        }
        // 预备条件准备好了

        // 循环判断
        i = size_a;
        while i < size_b {
            if bytes_b[i - size_a] != bytes_b[i] {
                return -1;
            }
            i += 1;
        }
        // a,b的长度均能达到10^4
        // 最后只需验证循环的确实是a而不是a的同构单词即可
        i = 0;
        let rest = if size_b % size_a > 0 { 1 } else { 0 };
        let mut j: usize;
        let count = size_b / size_a;
        let mut flag:bool;
        while i < size_a {
            // 前面多余的字母数量为i
            j = 0;
            flag = true;
            while j < size_a && j < size_b {
                if bytes_a[(j + i) % size_a] != bytes_b[j] {
                    flag = false;
                    break;
                }
                j += 1;
            }
            let head = size_a - i;
            if flag {
                if i == 0 {
                    return (count + rest) as i32;
                } else {
                    if head == size_b % size_a {
                        return (count + 1) as i32;
                    } else if size_b % size_a > head {
                        return (count + 2) as i32;
                    } else {
                        return (count + 1) as i32;
                    }
                }
            }
            i += 1;
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_686() {
        assert_eq!(3, Solution::repeated_string_match(String::from("abcd"), String::from("cdabcdab")));
        assert_eq!(4, Solution::repeated_string_match(String::from("abc"), String::from("cabcabca")));
    }
}