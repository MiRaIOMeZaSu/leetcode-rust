pub struct Solution {}

fn is_even(n: i32) -> bool {
    return n % 2 == 0;
}

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut left: i32 = 1;
        let mut gap: i32 = 1;
        let mut size: i32 = n;
        if n == 1 {
            return 1;
        }
        // 直接每次都计算起始点,数量和间隔
        while size > 1 {
            // 往右
            left += gap;
            gap *= 2;
            size /= 2;
            // 往左
            if size > 1 {
                if !is_even(size) {
                    left += gap;
                }
                gap *= 2;
                size/=2;
            }
        }
        return left;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_390() {
        // assert_eq!(6, Solution::last_remaining(9));
        // assert_eq!(1, Solution::last_remaining(1));
        // assert_eq!(2, Solution::last_remaining(4));
        // assert_eq!(4, Solution::last_remaining(6));
        assert_eq!(6, Solution::last_remaining(21));
        assert_eq!(54, Solution::last_remaining(100));
    }
}