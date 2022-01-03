pub struct Solution {}

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        // 直接算出1971到现在的天数
        // 1971年1月1日为星期五
        let map = vec!["Friday", "Saturday", "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday"];
        let mut ans: usize = usize::from((day - 1) as u16);
        let mut y = year - 1971;
        let y_times = y / 4;
        y %= 4;
        ans += usize::from((y_times * (365 * 3 + 366) + y * 365) as u16);
        if month >= 2 {
            ans += 31;
        }
        if month >= 3 {
            ans += 28;
            if year % 4 == 0 {
                ans += 1;
            }
        }
        if month >= 4 {
            ans += 31;
        }
        if month >= 5 {
            ans += 30;
        }
        if month >= 6 {
            ans += 31;
        }
        if month >= 7 {
            ans += 30;
        }
        if month >= 8 {
            ans += 31;
        }
        if month >= 9 {
            // 8月的
            ans += 31;
        }
        if month >= 10 {
            ans += usize::from((((month - 9) / 2) * 61 + ((month - 9) % 2) * 30) as u16);
        }
        ans %= 7;
        return map[ans].to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1185() {
        // assert_eq!("Friday", Solution::day_of_the_week(1, 1, 1971));
        // assert_eq!("Tuesday", Solution::day_of_the_week(30, 11, 1971));
        assert_eq!("Saturday", Solution::day_of_the_week(1, 1, 1972));
        assert_eq!("Sunday", Solution::day_of_the_week(1, 1, 1973));
        assert_eq!("Monday", Solution::day_of_the_week(1, 1, 1974));
        assert_eq!("Wednesday", Solution::day_of_the_week(1, 1, 1975));
        assert_eq!("Thursday", Solution::day_of_the_week(1, 1, 1976));
        // assert_eq!("Sunday", Solution::day_of_the_week(3, 1, 1972));
        // assert_eq!("Saturday", Solution::day_of_the_week(31, 8, 2019));
        // assert_eq!("Sunday", Solution::day_of_the_week(18, 7, 1999));
        assert_eq!("Friday", Solution::day_of_the_week(1, 1, 1993));
        // assert_eq!("Sunday", Solution::day_of_the_week(15, 8, 1993));
    }
}