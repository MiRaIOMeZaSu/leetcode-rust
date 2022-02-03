pub struct Solution {}

impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut tokens: Vec<&str> = sentence.split(" ").collect();
        let mut ans = 0;
        for token in tokens {
            ans += Solution::satify(token);
        }
        return ans;
    }

    fn satify(word: &str) -> i32 {
        let a: u8 = 'a' as u8;
        let z: u8 = 'z' as u8;
        let c1: u8 = '!' as u8;
        let c2: u8 = '.' as u8;
        let c3: u8 = ',' as u8;
        let c4: u8 = '-' as u8;
        let bytes = word.as_bytes();
        let size = bytes.len();
        if size == 0 {
            return 0;
        }
        let first = bytes[0];
        let mut underline_used: bool = false;
        if size != 1 && (first < a || first > z) {
            return 0;
        }
        for i in 1..(size - 1) {
            if bytes[i] == c4 {
                if underline_used {
                    return 0;
                }
                underline_used = true;
                if bytes[i - 1] < a || bytes[i - 1] > z ||(bytes[i + 1] < a || bytes[i + 1] > z){
                    return 0;
                }
                continue;
            }
            if bytes[i] < a || bytes[i] > z {
                return 0;
            }
        }
        let last = bytes[size - 1];
        if (last < a || last > z) && last != c1 && last != c2 && last != c3 {
            return 0;
        }
        return 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2047() {
        assert_eq!(0, Solution::count_valid_words(String::from("!this  1-s b8d!")));
        assert_eq!(6, Solution::count_valid_words(String::from("he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.")));
    }
}