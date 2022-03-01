use std::ops::Add;

struct Solution {}

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let vec1 = Solution::to_num(num1);
        let vec2 = Solution::to_num(num2);
        return Solution::solve(vec1[0], vec1[1], vec2[0], vec2[1]);
    }
    pub fn to_num(num: String) -> Vec<i32> {
        let mut vec: Vec<&str> = num.split('+').collect();
        let mut ans: Vec<i32> = vec![0; 2];
        ans[0] = String::from(vec[0]).parse::<i32>().unwrap();
        vec = vec[1].split('i').collect();
        ans[1] = String::from(vec[0]).parse::<i32>().unwrap();
        return ans;
    }
    pub fn solve(num1: i32, num2: i32, num3: i32, num4: i32) -> String {
        let mut ans1: i32 = 0;
        let mut ans2: i32 = 0;
        // 如何保证?
        ans1 += num1 * num3 - num2 * num4;
        ans2 += num1 * num4 + num2 * num3;
        return ans1.to_string().add("+").add(ans2.to_string().as_str()).add("i");
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_537() {
        // Solution::gray_code(2);
        assert_eq!(String::from("0+2i"), Solution::complex_number_multiply(String::from("1+1i"), String::from("1+1i")));
    }
}