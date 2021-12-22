pub struct Solution {}

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        // 寻找数字
        let mut ans: i32 = 0;
        let month = *(&date[5..7].parse::<i32>().unwrap());
        let day = *(&date[8..10].parse::<i32>().unwrap());
        let year = *(&date[0..4].parse::<i32>().unwrap());
        if month > 1 { ans += 31; }
        if month > 2 {
            if year % 4 == 0 {
                ans += 29;
            } else { ans += 28; }
        }
        if month > 3 { ans += 31; }
        if month > 4 { ans += 30; }
        if month > 5 { ans += 31; }
        if month > 6 { ans += 30; }
        if month > 7 { ans += 31; }
        if month > 8 { ans += 31; }
        if month > 9 { ans += 30; }
        if month > 10 { ans += 31; }
        if month > 11 { ans += 30; }
        ans += day;
        return ans;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::day_of_year(String::from("2019-01-09")));
        assert_eq!(60, Solution::day_of_year(String::from("2003-03-01")));
    }
}