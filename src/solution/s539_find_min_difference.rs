pub struct Solution {}

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        // 直接计算分钟数并排序即可
        let mut times: Vec<i32> = vec![];
        let mut ans: i32 = i32::MAX;
        for time in time_points {
            let hour = String::from(&time[0..2]).parse::<i32>().unwrap();
            let minute = String::from(&time[3..5]).parse::<i32>().unwrap();
            times.push(hour * 60 + minute);
        }
        times.sort();
        let mut last = -1;
        for i in 0..times.len() {
            let time = times[i];
            if last != -1 {
                let temp = time - last;
                if temp < ans {
                    ans = temp;
                }
            }
            last = time;
        }
        let temp = times[0] + (24 * 60 - last);
        if temp < ans {
            ans = temp;
        }
        return ans;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_539() {
        // assert_eq!(true, Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3));
        // assert_eq!(false, Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
        // assert_eq!(false, Solution::is_n_straight_hand(vec![8, 10, 12], 3));
        assert_eq!(1, Solution::find_min_difference(vec![String::from("23:59"), String::from("00:00")]));
    }
}