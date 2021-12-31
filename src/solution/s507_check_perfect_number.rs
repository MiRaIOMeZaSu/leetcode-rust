pub struct Solution {}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        // 从左到右
        let mut sum = -num;
        let mut left = 1;
        let mut right;
        loop {
            if num % left == 0 {
                right = num / left;
                if left < right {
                    sum += left + right;
                } else {
                    break;
                }
            }
            left += 1;
        }
        return sum == num;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_507() {
        assert_eq!(true,Solution::check_perfect_number(28));
        assert_eq!(true,Solution::check_perfect_number(6));
        assert_eq!(true,Solution::check_perfect_number(496));
        assert_eq!(true,Solution::check_perfect_number(8128));
        assert_eq!(false,Solution::check_perfect_number(2));
    }
}