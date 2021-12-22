pub struct Solution {}

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        // 重点是如何比较满足条件
        let mut ans = -1;
        let bytesA = a.as_bytes();
        let bytesB = b.as_bytes();
        let mut charCountA: [i32; 26] = [0; 26];
        let mut charCountBSizeA: [i32; 26] = [0; 26];
        let mut charCountB: [i32; 26] = [0; 26];
        let sizeA = a.len();
        let sizeB = b.len();
        let pivot: usize = usize::from(String::from("a").as_bytes()[0]);
        let mut i: usize = 0;
        while i < sizeA {
            charCountA[usize::from(bytesA[i]) - pivot] += 1;
            i += 1;
        }
        i = 0;
        while i < sizeA && i < sizeB {
            charCountBSizeA[usize::from(bytesB[i]) - pivot] += 1;
            i += 1;
        }
        i = 0;
        // 以单个单词为单位判断
        while sizeB > sizeA && i < 26 {
            if charCountA[i] != charCountBSizeA[i] {
                return -1;
            }
            i += 1;
        }
        i = 0;
        while i < sizeB {
            charCountB[usize::from(bytesB[i]) - pivot] += 1;
            i += 1;
        }
        // 预备条件准备好了

        // 循环判断
        i = sizeA;
        while i < sizeB {
            if bytesB[i - sizeA] != bytesB[i] {
                return -1;
            }
            i += 1;
        }
        // a,b的长度均能达到10^4
        // 最后只需验证循环的确实是a而不是a的同构单词即可
        i = 0;
        let rest = if sizeB % sizeA > 0 { 1 } else { 0 };
        let mut j: usize = 0;
        let mut flag = true;
        let count = sizeB / sizeA;
        while i < sizeA {
            // 前面多余的字母数量为i
            j = 0;
            flag = true;
            while j < sizeA && j < sizeB {
                if bytesA[(j + i) % sizeA] != bytesB[j] {
                    flag = false;
                    break;
                }
                j += 1;
            }
            let head = sizeA - i;
            if flag {
                if i == 0 {
                    return (count + rest) as i32;
                } else {
                    if head == sizeB % sizeA {
                        return (count + 1) as i32;
                    } else if sizeB % sizeA > head {
                        return (count + 2) as i32;
                    } else {
                        return (count + 1) as i32;
                    }
                }
            }
            i += 1;
        }
        return ans;
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