use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        return Solution::solve(s.as_bytes(), 0, s.len());
    }

    fn solve(bytes: &[u8], left: usize, right: usize) -> String {
        let mut map: HashMap<u8, Vec<usize>> = HashMap::new();
        for i in left..right {
            let mut indexs = map.entry(bytes[i]).or_insert(vec![]);
            indexs.push(i);
        }
        let mut div: Vec<usize> = vec![];
        for i in 0..26 {
            let small: u8 = i + 'a' as u8;
            let big: u8 = i + 'A' as u8;
            if (map.contains_key(&small) && !map.contains_key(&big)) ||
                (!map.contains_key(&small) && map.contains_key(&big))
            {
                div.append(map.entry(small).or_insert(vec![]));
                div.append(map.entry(big).or_insert(vec![]));
            }
        }
        let mut l = left;
        let mut r = right;
        let mut ans = String::new();
        if !div.is_empty() {
            div.push(right);
            div.sort();
            let size = div.len();
            for i in 0..size {
                r = div[i];
                if l + 1 < r {
                    let temp_ans = Solution::solve(bytes, l, r);
                    if temp_ans.len() > ans.len() {
                        ans = temp_ans;
                    }
                }
                l = r + 1;
            }
            return ans;
        }
        let mut sub_bytes = &bytes[left..r];
        return String::from_utf8_lossy(&sub_bytes).to_string();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1763() {
        // Solution::gray_code(2);
        assert_eq!(String::from("aAa"), Solution::longest_nice_substring(String::from("YazaAay")));
        // assert_eq!(String::from(""), Solution::longest_nice_substring(String::from("jcJ")));
        // assert_eq!(String::from("qQ"), Solution::longest_nice_substring(String::from("qQUjJ")));
    }
}