pub struct Solution {}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        // 如何使变更的只有一个数字
        let mut ans: Vec<i32> = vec![0, 1];
        if n == 1 {
            return ans;
        }
        let mut i = 1;
        let mut appender = 2;
        while i < n {
            let size = ans.len();
            let mut j = size;
            while j > 0 {
                ans.push(ans[j - 1] + appender);
                j -= 1;
            }
            i += 1;
            appender *= 2;
        }
        return ans;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_89() {
        // Solution::gray_code(2);
        Solution::gray_code(3);
    }
}