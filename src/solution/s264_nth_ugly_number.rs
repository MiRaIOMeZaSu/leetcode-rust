use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut a: i32 = 1;
        let mut b: i32 = 1;
        let mut c: i32 = 1;
        let final_a = 2;
        let final_b = 3;
        let final_c = 5;
        let mut i = 1;
        let mut next_a = a * final_a;
        let mut next_b = b * final_b;
        let mut next_c = c * final_c;
        while i < n {
            let min_num = cmp::min(next_a, cmp::min(next_b, next_c));
            if min_num == next_a {
                a = next_a;
                next_a *= final_a;
            } else if min_num == b * final_b {
                b = next_b;
                next_b *= final_b;
            } else {
                c = next_c;
                next_c *= final_c;
            }
            i += 1;
        }
        return a * b * c;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_264() {
        assert_eq!(12, Solution::nth_ugly_number(10));
        assert_eq!(1, Solution::nth_ugly_number(1));
    }
}