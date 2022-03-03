struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        // 变成string
        if num < 10 {
            return num;
        }
        let mut curr_num = num;
        let mut next_num = 0;
        // 如何计算
        while curr_num > 0 {
            next_num += curr_num % 10;
            curr_num /= 10;
        }
        return Solution::add_digits(next_num);
    }
}