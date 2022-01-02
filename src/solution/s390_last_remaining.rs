pub struct Solution {}

fn is_even(n: i32) -> bool {
    return n % 2 == 0;
}

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut mutiple: i32 = 1;
        let mut divide: i32 = 1;
        let mut left: i32 = n;
        if n == 1 {
            return 1;
        }
        let mut toRightNow: bool = true;
        while left >= 4 {
            // 往右
            mutiple *= 2;
            left /= 2;
            // 往左
            toRightNow = false;
            if left >= 4 {
                if is_even(left) {
                    // 移除的是偶数位
                    divide *= 2;
                } else {
                    mutiple *= 2;
                }
                left /= 2;
                toRightNow = true;
            }
        }
        let mut ans: i32 = mutiple;
        if left == 3 {
            ans += divide * mutiple;
        } else if left == 2 {
            if toRightNow {
                ans += divide * mutiple;
            }
        }
        return ans;
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